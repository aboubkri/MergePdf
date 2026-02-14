use std::collections::BTreeMap;
use std::path::Path;

use lopdf::{dictionary, Document, Object, ObjectId};

/// Merge a list of PDF files into output_path.
/// files must contain at least one entry.
pub fn merge_multiple_pdfs<P: AsRef<Path>>(files: &[P], output_path: P) -> lopdf::Result<()> {
    if files.is_empty() {
        // Create an io::Error and convert it into lopdf::Error via .into()
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No input PDFs provided",
        ).into());
    }

    // Load all documents and renumber them so object ids don't collide.
    // We'll collect pages separately (so we can build a single /Pages root),
    // and collect all other objects into documents_objects.
    let mut max_id: u32 = 1; // starting id for renumbering
    let mut documents_pages: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut documents_objects: BTreeMap<ObjectId, Object> = BTreeMap::new();
    
    for file in files {
        let mut doc = Document::load(file)?;

        // Renumber the object's ids in-place to start at max_id.
        doc.renumber_objects_with(max_id);

        // advance max_id for the next document
        max_id = doc.max_id + 1;

        // collect pages: get_pages returns a BTreeMap<page_number, ObjectId>
        // map to (ObjectId, Object) pairs (we clone the object)
        documents_pages.extend(
            doc.get_pages()
                .into_iter()
                .map(|(_, object_id)| {
                    (
                        object_id,
                        doc.get_object(object_id).unwrap().to_owned(),
                    )
                })
                .collect::<BTreeMap<ObjectId, Object>>()
        );

        // collect all objects (we will skip Page objects later when building)
        documents_objects.extend(doc.objects);
    }

    // Build a fresh new document to hold everything
    let mut out_doc = Document::with_version("1.5");

    // We'll need to pick a Pages object id and Catalog id. If any of the input
    // documents had a Catalog/Pages we kept their IDs during renumbering, so
    // we may reuse the first Catalog/Pages we encounter. We'll collect them and
    // then override fields we need.
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Insert all non-Page objects into out_doc except Pages/Catalog/Outlines/Outline.
    for (object_id, object) in documents_objects.iter() {
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                // keep first Catalog we find
                if catalog_object.is_none() {
                    catalog_object = Some((*object_id, object.clone()));
                }
            }
            b"Pages" => {
                // Keep/merge first Pages object we find
                if let Ok(dict) = object.as_dict() {
                    let mut dict = dict.clone();
                    if let Some((_, ref existing)) = pages_object {
                        if let Ok(old_dict) = existing.as_dict() {
                            // merge dictionaries: new entries extend old
                            dict.extend(old_dict);
                        }
                    }
                    if pages_object.is_none() {
                        pages_object = Some((*object_id, Object::Dictionary(dict)));
                    } else {
                        // if already present, update the stored dict (merged above)
                        pages_object = Some((pages_object.unwrap().0, Object::Dictionary(dict)));
                    }
                }
            }
            b"Page" => {
                // skip -- handled later
            }
            b"Outlines" | b"Outline" => {
                // skip outlines/outline (optional: implement if you want to preserve bookmarks)
            }
            _ => {
                // other objects (fonts, resources, XObjects, etc.) go into out_doc
                out_doc.objects.insert(*object_id, object.clone());
            }
        }
    }

    // ensure we found a pages object (if none found in inputs, create one)
    if pages_object.is_none() {
        let pages_id = out_doc.new_object_id();
        let pages_dict = dictionary! {
            "Type" => "Pages",
            "Kids" => Vec::<Object>::new(),
            "Count" => 0u32,
        };
        out_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
        pages_object = Some((pages_id, out_doc.objects.get(&pages_id).unwrap().clone()));
    }

    // Insert page objects (fix their Parent to the chosen Pages id)
    let pages_root_id = pages_object.as_ref().unwrap().0;
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dict) = object.as_dict() {
            let mut dict = dict.clone();
            dict.set("Parent", pages_root_id);
            out_doc.objects.insert(*object_id, Object::Dictionary(dict));
        } else {
            // If page is not a dict (shouldn't happen), insert as-is
            out_doc.objects.insert(*object_id, object.clone());
        }
    }

    // Build the new Pages dictionary: set Count and Kids list
    let kids: Vec<_> = documents_pages
        .keys()
        .map(|oid| Object::Reference(*oid))
        .collect();

    if let Ok(mut pages_dict) = pages_object.unwrap().1.as_dict().cloned() {
        pages_dict.set("Count", kids.len() as u32);
        pages_dict.set("Kids", kids);
        out_doc.objects.insert(pages_root_id, Object::Dictionary(pages_dict));
    }

    // Build / set Catalog: use a catalog from inputs if found, else create one
    let catalog_id = if let Some((id, catalog_obj)) = catalog_object {
        // ensure it points to our pages root
        if let Ok(mut dict) = catalog_obj.as_dict().cloned() {
            dict.set("Pages", pages_root_id);
            dict.remove(b"Outlines"); // optional: drop outlines
            out_doc.objects.insert(id, Object::Dictionary(dict));
            id
        } else {
            // fallback: create a fresh catalog
            let new_catalog_id = out_doc.new_object_id();
            let cat_dict = dictionary! { "Type" => "Catalog", "Pages" => pages_root_id };
            out_doc.objects.insert(new_catalog_id, Object::Dictionary(cat_dict));
            new_catalog_id
        }
    } else {
        // no catalog in inputs -> create one
        let new_catalog_id = out_doc.new_object_id();
        let cat_dict = dictionary! { "Type" => "Catalog", "Pages" => pages_root_id };
        out_doc.objects.insert(new_catalog_id, Object::Dictionary(cat_dict));
        new_catalog_id
    };

    // Set trailer root
    out_doc.trailer.set("Root", catalog_id);

    // Update max_id and renumber objects for cleanliness
    out_doc.max_id = out_doc.objects.keys().map(|(id, _gen)| *id).max().unwrap_or(0);
    out_doc.renumber_objects();
    out_doc.compress();

    // Save merged file
    out_doc.save(output_path)?;

    Ok(())
}

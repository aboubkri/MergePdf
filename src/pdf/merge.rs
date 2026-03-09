use std::collections::BTreeMap;
use std::path::Path;

use lopdf::{dictionary, Document, Object, ObjectId};
use tracing::{info, debug, warn};

use crate::error::MergeError;

/// Merge a list of PDF files into output_path.
/// files must contain at least one entry.
pub fn merge_multiple_pdfs<P: AsRef<Path>>(files: &[P], output_path: P) -> Result<(), MergeError> {
    if files.is_empty() {
        return Err(MergeError::NoInputFiles);
    }

    let mut max_id: u32 = 1;
    let mut documents_pages: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut documents_objects: BTreeMap<ObjectId, Object> = BTreeMap::new();
    
    for file in files {
        let path_ref = file.as_ref();
        info!("Loading PDF: {:?}", path_ref);
        let mut doc = Document::load(path_ref)?;

        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        // Collect pages safely without unwrap
        for (_, object_id) in doc.get_pages() {
            let obj = doc.get_object(object_id)?.to_owned();
            documents_pages.insert(object_id, obj);
        }

        documents_objects.extend(doc.objects);
    }

    info!("Building merged document...");
    let mut out_doc = Document::with_version("1.5");

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects.iter() {
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                if catalog_object.is_none() {
                    catalog_object = Some((*object_id, object.clone()));
                }
            }
            b"Pages" => {
                if let Ok(dict) = object.as_dict() {
                    let mut dict = dict.clone();
                    if let Some((_, ref existing)) = pages_object {
                        if let Ok(old_dict) = existing.as_dict() {
                            dict.extend(old_dict);
                        }
                    }
                    if let Some((id, _)) = pages_object {
                        pages_object = Some((id, Object::Dictionary(dict)));
                    } else {
                        pages_object = Some((*object_id, Object::Dictionary(dict)));
                    }
                }
            }
            b"Page" | b"Outlines" | b"Outline" => { /* Skip */ }
            _ => {
                out_doc.objects.insert(*object_id, object.clone());
            }
        }
    }

    // Ensure we found a pages object
    let (pages_root_id, pages_obj) = if let Some(po) = pages_object {
        po
    } else {
        warn!("No Pages object found in inputs; creating a new one.");
        let pages_id = out_doc.new_object_id();
        let pages_dict = dictionary! {
            "Type" => "Pages",
            "Kids" => Vec::<Object>::new(),
            "Count" => 0u32,
        };
        out_doc.objects.insert(pages_id, Object::Dictionary(pages_dict.clone()));
        (pages_id, Object::Dictionary(pages_dict))
    };

    // Insert page objects
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dict) = object.as_dict() {
            let mut dict = dict.clone();
            dict.set("Parent", pages_root_id);
            out_doc.objects.insert(*object_id, Object::Dictionary(dict));
        } else {
            out_doc.objects.insert(*object_id, object.clone());
        }
    }

    // Build the new Pages dictionary
    let kids: Vec<_> = documents_pages.keys().map(|oid| Object::Reference(*oid)).collect();

    if let Ok(mut pages_dict) = pages_obj.as_dict().cloned() {
        pages_dict.set("Count", kids.len() as u32);
        pages_dict.set("Kids", kids);
        out_doc.objects.insert(pages_root_id, Object::Dictionary(pages_dict));
    }

    // Build Catalog
    let catalog_id = if let Some((id, catalog_obj)) = catalog_object {
        if let Ok(mut dict) = catalog_obj.as_dict().cloned() {
            dict.set("Pages", pages_root_id);
            dict.remove(b"Outlines");
            out_doc.objects.insert(id, Object::Dictionary(dict));
            id
        } else {
            let new_catalog_id = out_doc.new_object_id();
            let cat_dict = dictionary! { "Type" => "Catalog", "Pages" => pages_root_id };
            out_doc.objects.insert(new_catalog_id, Object::Dictionary(cat_dict));
            new_catalog_id
        }
    } else {
        let new_catalog_id = out_doc.new_object_id();
        let cat_dict = dictionary! { "Type" => "Catalog", "Pages" => pages_root_id };
        out_doc.objects.insert(new_catalog_id, Object::Dictionary(cat_dict));
        new_catalog_id
    };

    out_doc.trailer.set("Root", catalog_id);
    
    // Clean up and save
    debug!("Compressing and saving document...");
    out_doc.max_id = out_doc.objects.keys().map(|(id, _)| *id).max().unwrap_or(0);
    out_doc.renumber_objects();
    out_doc.compress();

    out_doc.save(output_path.as_ref())?;
    info!("Successfully merged into {:?}", output_path.as_ref());

    Ok(())
}
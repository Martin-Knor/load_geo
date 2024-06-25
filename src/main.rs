use std::fs::File;
use std::io::Read;
use geo_types::{Geometry, GeometryCollection};
use geojson::{quick_collection, GeoJson};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the GeoJSON file
    let mut file = File::open("src/test.geojson")?;
    
    // Read the file contents into a string
    let mut geojson_string = String::new();
    file.read_to_string(&mut geojson_string)?;
    
    // Parse the GeoJSON string
    let geojson = geojson_string.parse::<GeoJson>()?;
    
    // transform contents of GeoJason into types supported by geo-types crtate
    // https://docs.rs/geojson/0.22.4/geojson/index.html#use-geojson-with-other-crates-by-converting-to-geo-types
    let collection: GeometryCollection<f64> = quick_collection(&geojson).unwrap();
    
    // Process the GeoJSON data
    match geojson {
        //if geojson is FeatureCollection type loop over its features
        GeoJson::FeatureCollection(collection) => {
            for feature in collection.features {
                println!("{:?}", feature);
            }
        },
        //if geojson is Feature type 
        GeoJson::Feature(feature) => {
            println!("{:?}", feature);
        },
        //if geojson is Geometry type 
        GeoJson::Geometry(geometry) => {
            println!("{:?}", geometry);
        },
    }
    
    
    println!("\n\n");


    // Print the contents of the GeometryCollection
    for geometry in &collection {
        match geometry {
            Geometry::Point(point) => println!("Point: {:?}", point),
            Geometry::Polygon(polygon) => println!("Polygon: {:?}", polygon),
            Geometry::MultiPolygon(multi_polygon) => println!("MultiPolygon: {:?}", multi_polygon),
            Geometry::GeometryCollection(geometry_collection) => {
                println!("GeometryCollection: {:?}", geometry_collection)
            },
            _ => return Err("Unsupported geometry type found".into()),
        }
    }

    Ok(())
}

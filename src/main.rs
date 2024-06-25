use std::fs::File;
use std::io::Read;
use geojson::GeoJson;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the GeoJSON file
    let mut file = File::open("src/test.geojson")?;
    
    // Read the file contents into a string
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    
    // Parse the GeoJSON string
    let geojson = data.parse::<GeoJson>()?;
    
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

    Ok(())
}

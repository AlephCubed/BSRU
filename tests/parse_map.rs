use bsru::difficulty::Difficulty;
use bsru::info::Beatmap;
use std::fs;

#[test]
fn parse_beatmaps() {
    let paths = fs::read_dir("test_maps").unwrap().filter_map(|result| {
        if let Ok(dir) = result {
            if dir.path().is_dir() {
                return Some(dir.path());
            }
        }
        None
    });

    for path in paths {
        println!("{path:?}");
        let mut info_path = path.clone();
        info_path.push("Info.dat");

        let info_file = fs::File::open(&info_path).expect("Map missing info file");
        let map: Beatmap = serde_json::from_reader(info_file).expect("Invalid info file");

        for set in map.difficulty_sets {
            println!("\t{:?}", set.characteristic);

            for dif in set.difficulties {
                println!("\t\t{} ({:?})", dif.name, dif.rank);

                let mut dif_path = path.clone();
                dif_path.push(dif.file);

                let dif_file = fs::File::open(&dif_path).expect("Map missing difficulty file");
                let _: Difficulty =
                    serde_json::from_reader(dif_file).expect("Invalid difficulty file");
            }
        }
    }
}

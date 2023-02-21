use std::io::Seek;

fn step_01() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("A.ARW")?;
    let mut buf_reader = std::io::BufReader::new(&file);

    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut buf_reader)?;

    for f in exif.fields() {
        println!(
            "{}, {}, {}",
            f.tag,
            f.ifd_num,
            f.display_value().with_unit(&exif)
        );
    }

    println!("-- PART-2 --");
    file.seek(std::io::SeekFrom::Start(0))?;
    let mut buf_reader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut buf_reader)?;
    for f in exif.fields() {
        println!(
            "{}/{:?}/{} __ {} __ {}",
            f.tag,
            f.tag.context(),
            f.tag.number(),
            f.ifd_num,
            f.display_value().with_unit(&exif)
        );
    }

    let f0 = exif
        .get_field(exif::Tag::DateTimeDigitized, exif::In::PRIMARY)
        .unwrap();

    println!("-- PART-3 --");
    println!("{}", f0.display_value().with_unit(&exif));
    println!("{:?}", f0.value);
    Ok(())
}

fn step_02() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- PART-4 --");
    let exif = rexif::parse_file("A.ARW")?;
    for entry in &exif.entries {
        println!("{}: {}", entry.tag, entry.value_more_readable);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    step_01()?;
    step_02()
}

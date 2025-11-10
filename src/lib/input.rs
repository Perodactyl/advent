use std::fs;

fn setup_input_dir(year: usize) -> std::io::Result<()> {
	if !fs::exists("input")? {
		fs::create_dir("input")?;
	}
	if !fs::exists(format!("input/{year}"))? {
		fs::create_dir(format!("input/{year}"))?;
	}
	Ok(())
}

pub fn input(year: usize, day: usize) -> Result<String, reqwest::Error> {
	assert!(year >= 2015);
	assert!(day >= 1 && day <= 25);
	setup_input_dir(year).unwrap();
	
	match fs::read_to_string(format!("input/{year}/day{day}.input")) {
		Ok(content) => return Ok(content),
		Err(e) if e.kind() == std::io::ErrorKind::NotFound => {},
		Err(e) => eprintln!("{e:?}"),
	}

	let client = reqwest::blocking::Client::new();
	let r = client.request(reqwest::Method::GET, format!("https://adventofcode.com/{year}/day/{day}/input"))
		.header("Cookie", format!("session={}", std::env::var("token").unwrap()))
		.build()?;

	let response = client.execute(r)?.error_for_status()?;
	let text = response.text()?.trim().to_string();

	match fs::write(format!("input/{year}/day{day}.input"), &text) {
		Ok(()) => {},
		Err(e) => eprintln!("{e:?}"),
	}

	Ok(text)
}

pub fn sample(year: usize, day: usize) -> Result<String, std::io::Error> {
	assert!(year >= 2015);
	assert!(day >= 1 && day <= 25);
	setup_input_dir(year)?;
	let path = format!("input/{year}/day{day}.sample{}", if cfg!(feature = "sample2") { "2" } else { "" });
	if !fs::exists(&path)? {
		return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("{path} not found. Please paste sample data in.")));
	}

	Ok(fs::read_to_string(path)?.trim().to_string())
}

use chrono::Datelike;
pub fn std_vars() -> Vec<Vec<String>>{
	let vars = [
		["version", std::env!("CARGO_PKG_VERSION")],
		["year", &chrono::Utc::now().year().to_string()],
		["month", &chrono::Utc::now().month().to_string()],
		["day", &chrono::Utc::now().day().to_string()],
		["invbang", "¡"],
		["copy", "©"],
		["reg", "®"],
		["deg", "°"],
		["micro", "µ"],
	
	//these are unicode combining chars, you can't see them, but they're there!
		["accute", " ́"],
		["overline", " ̅"],
		["diaresis", " ̈"],
	
	//math
		["theta", "ϴ"],
		["omega", "Ω"],
		["alpha", "α"],
		["beta", "β"],
		["gamma", "γ"],
		["delta", "Δ"],
		["sigma", "Σ"],
		["pi", "π"],
		["cap_omega", "Ѡ"],
		["bullet", "•"],
		["div", "÷"],
		["mul", "×"],
		["plusmn", "±"],

		["block", "█"],
		["light_shade", "░"],
		["med_shade", "▒"],
		["dark_shade", "▓"],
	//emojis!!!!
		["thunder", "⚡"],
		["fist", "✊"],
		["check", "✔"],
		["x", "✘"],
		["sparkle", "✨"],
		["cross", "❌"],
		["black_heart", "❤"],
		["smile", "😀"],
		["veryfunny", "😂"],
		["smile2", "😃"],
		["embarrassment", "😅"],
		["cool", "😎"],
		["smirk", "😏"],
		["apathy", "😐"],
		["crying", "😢"],
		["ono", "😳"],
		["Smile", "🙂"],
		["nerd", "🤓"],
		["brain", "🧠"],
		["cap", "🧢"],
		["frog", include_str!("sexy frog.txt")],
	];

	let mut out = Vec::new();
	for item in vars{
		let mut toadd = Vec::new();
		for i in item{
			toadd.push(String::from(i));
		}
        out.push(toadd);
	}
	return out;
}

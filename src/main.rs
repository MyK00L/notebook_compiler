use serde::Deserialize;
use std::collections::HashMap;
use std::io::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum PaperType {
    a0paper,
    a1paper,
    a2paper,
    a3paper,
    a4paper,
    a5paper,
    a6paper,
    b0paper,
    b1paper,
    b2paper,
    b3paper,
    b4paper,
    b5paper,
    b6paper,
    c0paper,
    c1paper,
    c2paper,
    c3paper,
    c4paper,
    c5paper,
    c6paper,
    b0j,
    b1j,
    b2j,
    b3j,
    b4j,
    b5j,
    b6j,
    ansiapaper,
    ansibpaper,
    ansicpaper,
    ansidpaper,
    ansiepaper,
    letterpaper,
    executivepaper,
    legalpaper,
}
impl std::fmt::Display for PaperType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize)]
enum FrameType {
    none,
    leftline,
    topline,
    bottomline,
    lines,
    single,
}
impl std::fmt::Display for FrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Config {
    layout: PaperType,
    landscape: bool,
    lmargin: String,       // left margin
    rmargin: String,       // right margin
    tmargin: String,       // top margin
    bmargin: String,       // bottom marin
    headrulewidth: String, // head rule thickness
    footrulewidth: String, // foot rule thickness
    headheight: String,    // head height
    headsep: String,       // head separation
    footskip: String,      // foot separation
    title: String,
    author: String,
    university: String, // university name
    team: String,       // team name
    ncols: u8,          // number of columns for code
    ncolstoc: u8,       // number of columns for table of contents
    columnsep: String,  // columns separation
    separatetoc: bool,  // wether to put the table of contents on a separate page
    fontsize: String,
    // minted options:
    tabsize: u8,
    linenos: bool,
    numbersep: String,     // numbers distance in mm
    mathescape: bool,      // use math in comments
    autogobble: bool,      // remove unecessary whitespace
    showspaces: bool,      // render spaces with another character
    showtabs: bool,        // render tabs with another character
    breaklines: bool,      // allow linebreaks
    breakanywhere: bool,   // allow linebreaks anywhere
    breakautoindent: bool, // indent broken lines
    frame: FrameType,      //none,leftline,topline,bottomline,lines,single
    framesep: String,      // distance between frame and content
    framerule: String,     // frame thickness
    style: String,         // pygment style
}

#[derive(Debug, Clone)]
enum LineType {
    Section(String),
    SubSection(String),
    File(String),
}

fn get_extension(s: &str) -> String {
    s.split('.').last().unwrap().to_string()
}

fn decode_layout(s: &str) -> Vec<LineType> {
    s.split('\n')
        .map(|x| x.trim_end())
        .filter(|x| !x.is_empty())
        .map(|x| {
            if let Some('\t') = x.chars().nth(1) {
                // 2 tabs: file
                LineType::File(x.trim().to_string())
            } else if let Some('\t') = x.chars().next() {
                // 1 tab: subsection
                LineType::SubSection(x.trim().to_string())
            } else {
                // 0 tabs: section
                LineType::Section(x.trim().to_string())
            }
        })
        .collect()
}

fn escape(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => res.push_str(r#"\&"#),
            '%' => res.push_str(r#"\%"#),
            '$' => res.push_str(r#"\$"#),
            '#' => res.push_str(r#"\#"#),
            '_' => res.push_str(r#"\_"#),
            '{' => res.push_str(r#"\{"#),
            '}' => res.push_str(r#"\}"#),
            '~' => res.push_str(r#"\texttt{\~{}}"#),
            '^' => res.push_str(r#"\^{}"#),
            '\\' => res.push_str(r#"$\backslash$"#),
            _ => res.push(c),
        }
    }
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = ron::de::from_reader(std::fs::File::open("config.ron")?)?;
    let layout = decode_layout(&std::fs::read_to_string("layout.txt")?);
    let mut extensions: HashMap<String, String> = HashMap::new();
    if config.layout != PaperType::a4paper && config.layout != PaperType::letterpaper {
        eprintln!("Warning: by ICPC rules notebook must be either a4 or letter paper");
    }
    for i in layout.iter() {
        if let LineType::File(x) = i {
            let ext = get_extension(x);
            if !ext.is_empty() && !extensions.contains_key(&ext) {
                extensions.insert(
                    ext,
                    String::from_utf8(
                        std::process::Command::new("pygmentize")
                            .arg("-N")
                            .arg(&x)
                            .output()?
                            .stdout,
                    )?
                    .trim()
                    .to_string(),
                );
            }
        }
    }
    let tabspaces = String::from_utf8(vec![32; config.tabsize.into()]).unwrap();
    let mut tex = std::io::BufWriter::new(std::fs::File::create("out.tex")?);
    writeln!(
        tex,
        "% this document was automatically generated by notebook_compiler made by MyK_00L"
    )?;
    writeln!(
        tex,
        "\\documentclass[{},oneside]{{amsart}}",
        config.fontsize
    )?;
    writeln!(tex,"\\usepackage[paper={},lmargin={},rmargin={},tmargin={},bmargin={},foot=0pt,landscape={}]{{geometry}}",config.layout,config.lmargin,config.rmargin,config.tmargin,config.bmargin,config.landscape)?;
    writeln!(tex, "\\usepackage{{fancyhdr}}")?;
    writeln!(tex, "\\usepackage[utf8]{{inputenc}}")?;
    if config.ncols > 1 || config.ncolstoc > 1 {
        writeln!(tex, "\\usepackage{{multicol}}")?;
    }
    writeln!(tex, "\\usepackage{{minted}}")?;
    writeln!(tex, "\\usepackage{{datetime}}")?;
    writeln!(tex, "\\usepackage[scaled]{{berasans}}")?;
    writeln!(tex, "\\usepackage[scaled]{{beramono}}")?;
    writeln!(tex, "\\renewcommand*\\familydefault{{\\sfdefault}}")?;
    writeln!(tex, "\\usepackage[T1]{{fontenc}}")?;
    writeln!(tex, "\\pagestyle{{fancy}}")?;
    writeln!(tex, "\\lhead{{{} - {}}}", config.university, config.team)?;
    writeln!(tex, "\\rhead{{Page: \\thepage}}")?;
    writeln!(tex, "\\cfoot{{}}")?;
    writeln!(
        tex,
        "\\renewcommand{{\\headrulewidth}}{{{}}}",
        config.headrulewidth
    )?;
    writeln!(
        tex,
        "\\renewcommand{{\\footrulewidth}}{{{}}}",
        config.footrulewidth
    )?;
    writeln!(tex, "\\setlength{{\\headheight}}{{{}}}", config.headheight)?;
    writeln!(tex, "\\setlength{{\\headsep}}{{{}}}", config.headsep)?;
    writeln!(tex, "\\setlength{{\\footskip}}{{{}}}", config.footskip)?;
    writeln!(tex, "\\setlength{{\\columnsep}}{{{}}}", config.columnsep)?;
    writeln!(tex, "\\title{{{}}}", config.title)?;
    writeln!(tex, "\\author{{{}}}", config.author)?;
    writeln!(tex, "\\date{{\\ddmmyyyydate{{\\today{{}}}}}}")?;
    for i in extensions.iter() {
        writeln!(tex,"\\newminted{{{}}}{{tabsize={},linenos={},mathescape={},autogobble={},showspaces={},showtabs={},breaklines={},breakanywhere={},breakautoindent={},frame={},framerule={},style={},numbersep={},framesep={}}}",i.1,config.tabsize,config.linenos,config.mathescape,config.autogobble,config.showspaces,config.showtabs,config.breaklines,config.breakanywhere,config.breakautoindent,config.frame,config.framerule,config.style,config.numbersep,config.framesep)?;
    }
    writeln!(tex, "\\begin{{document}}")?;
    writeln!(tex, "\\thispagestyle{{fancy}}")?;
    if config.ncolstoc > 1 {
        writeln!(tex, "\\begin{{multicols*}}{{{}}}", config.ncolstoc)?;
    }
    writeln!(tex, "\\tableofcontents")?;
    if config.ncolstoc > 1 && config.ncolstoc != config.ncols {
        writeln!(tex, "\\end{{multicols*}}")?;
    }
    if config.separatetoc {
        writeln!(tex, "\\newpage")?;
    }
    if config.ncols > 1 && config.ncolstoc != config.ncols {
        writeln!(tex, "\\begin{{multicols*}}{{{}}}", config.ncols)?;
    }

    for layout_line in layout {
        match layout_line {
            LineType::Section(x) => {
                writeln!(tex, "\\section{{{}}}", escape(&x))?;
            }
            LineType::SubSection(x) => {
                writeln!(tex, "\\subsection{{{}}}", escape(&x))?;
            }
            LineType::File(x) => {
                let ext = get_extension(&x);
                writeln!(
                    tex,
                    "\\begin{{{}code}}",
                    extensions.get(&ext).unwrap_or(&String::from("text"))
                )?;
                writeln!(
                    tex,
                    "{}",
                    std::fs::read_to_string(&x)?.replace('\t', &tabspaces)
                )?;
                writeln!(
                    tex,
                    "\\end{{{}code}}",
                    extensions.get(&ext).unwrap_or(&String::from("text"))
                )?;
            }
        }
    }

    if config.ncols > 1 {
        writeln!(tex, "\\end{{multicols*}}")?;
    }
    writeln!(tex, "\\end{{document}}")?;
    Ok(())
}

// shared helpers of the tests (included by path)
use fx_eval::{build_effect, Corpus};
use fx_exec::exec::*;
pub fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
pub const BO: &str = "Blur_Motion_Curves_No_Transform_Autogen";
pub const TB: &str = "Transform_and_Blur_Autogen";
pub fn binding(blur_res: f32, postdiv: bool) -> Binding {
    let mut b = Binding::defaults();
    b.overrides.insert("blur_res".into(), vec![blur_res]);
    b.do_postdiv = Some(postdiv);
    b.hc_zero_legacy = true;
    b.host.insert("get_curve_val".into(), vec![0.3, 0.2, 0.1, 0.4]);
    b.const_inputs.insert("Mask".into(), vec![1.0]);
    b
}
pub fn program(name: &str) -> fx_eval::build::Genotype {
    let t = std::fs::read_to_string(pt()).unwrap();
    let c = Corpus::from_programs_tsv(&t);
    let ei = c.find_effect(name, None).unwrap();
    build_effect(&c, ei).unwrap()
}
pub fn plan(name: &str, b: &Binding) -> String {
    let cp = compile_for(&program(name), b).unwrap();
    fx_eval::cpuplan::plan_text_mode(&cp, fx_eval::cpuplan::PlanMode::CpuDriver)
}
/// the command lines of a plan text with every ref id replaced by its order of first appearance
pub fn canon(text: &str) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    text.lines()
        .filter(|l| l.starts_with("CMD"))
        .map(|l| {
            l.replace("\\n", " ")
                .split_whitespace()
                .map(|t| {
                    if let Some(r) = t.strip_prefix(":ref") {
                        let id = r.split('.').next().unwrap().to_string();
                        let k = seen.iter().position(|x| *x == id).unwrap_or_else(|| { seen.push(id); seen.len() - 1 });
                        format!(":r{}", k)
                    } else {
                        t.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}
pub fn run(name: &str, b: &Binding, w: usize, h: usize, alpha: &dyn Fn(usize) -> f32) -> Vec<f32> {
    let g = program(name);
    let mut planes = vec![vec![0.0f32; w * h]; 4];
    for i in 0..w * h {
        let a = alpha(i);
        let c = ((i * 7 % 13) as f32) / 13.0;
        planes[0][i] = c * a;
        planes[1][i] = (1.0 - c) * a;
        planes[2][i] = 0.25 * a;
        planes[3][i] = a;
    }
    let mut images = std::collections::BTreeMap::new();
    images.insert("Source".to_string(), fx_exec::types::Image { n: w * h, planes });
    let inputs = Inputs { width: w, height: h, images };
    let out = execute(&g, b, &inputs, &HostFrames::default()).unwrap();
    out.dest.planes.concat()
}

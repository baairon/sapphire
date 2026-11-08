//! Hand-evaluated cases of the `append` entry: planes of the first argument, then the second, at most four.
use ops_vec::*;

fn img<'a>(vals: &'a [u32], planes: &'a [&'a [f32]], maxch: i32) -> Block<'a> {
    Block { is_int: false, values: vals, enable: false, maxch, img_valid: true, planes }
}

fn cst<'a>(vals: &'a [u32]) -> Block<'a> {
    Block { is_int: false, values: vals, enable: true, maxch: 0, img_valid: true, planes: &[] }
}

fn params(n: i32) -> Params {
    Params { host: Host { in_pool: false }, roi: Roi { mode: 0, n, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0 } }
}

#[test]
fn two_planes_then_constant() {
    let (p0, p1) = ([1.0f32, 2.0, 3.0], [4.0f32, 5.0, 6.0]);
    let planes: [&[f32]; 2] = [&p0, &p1];
    let z = [0u32; 4];
    let c = [0.1f32.to_bits(), 0, 0, 0];
    let blocks = [img(&z, &planes, 1), cst(&c)];
    let mut o: Vec<Vec<f32>> = vec![vec![-1.0; 3]; 4];
    {
        let mut refs: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        append::render(&params(3), &blocks, 2, 1, &mut refs).unwrap();
    }
    assert_eq!(o[0], p0);
    assert_eq!(o[1], p1);
    assert_eq!(o[2], vec![0.1f32; 3]);
    assert_eq!(o[3], vec![-1.0f32; 3], "the fourth plane is untouched");
}

#[test]
fn writes_at_most_four_planes() {
    let p = [7.0f32, 8.0];
    let planes: [&[f32]; 1] = [&p];
    let z = [0u32; 4];
    let c = [2.5f32.to_bits(), 0, 0, 0];
    let blocks = [img(&z, &planes, 0), cst(&c)];
    // first argument claims three planes, second claims three: only one of the second fits
    let planes3: [&[f32]; 3] = [&p, &p, &p];
    let blocks3 = [img(&z, &planes3, 2), cst(&c)];
    let mut o: Vec<Vec<f32>> = vec![vec![0.0; 2]; 4];
    let mut refs: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
    append::render(&params(2), &blocks3, 3, 3, &mut refs).unwrap();
    drop(refs);
    assert_eq!(o[3], vec![2.5f32; 2]);
    let _ = blocks;
}

#[test]
fn first_argument_alone_and_empty() {
    let p = [1.5f32, 2.5];
    let planes: [&[f32]; 1] = [&p];
    let z = [0u32; 4];
    let c = [9.0f32.to_bits(), 0, 0, 0];
    let blocks = [img(&z, &planes, 0), cst(&c)];
    let mut o: Vec<Vec<f32>> = vec![vec![-3.0; 2]; 4];
    {
        let mut refs: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        let acts = append::render(&params(2), &blocks, 1, 0, &mut refs).unwrap();
        assert_eq!(acts.len(), 1);
    }
    assert_eq!(o[0], p);
    assert_eq!(o[1], vec![-3.0f32; 2]);
    let mut refs: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
    assert!(append::render(&params(2), &blocks, 0, 0, &mut refs).unwrap().is_empty());
}

#[test]
fn too_few_blocks_or_planes_is_an_error() {
    let z = [0u32; 4];
    let c = [1u32; 4];
    let one = [cst(&c)];
    let mut o: Vec<Vec<f32>> = vec![vec![0.0; 1]; 2];
    let mut refs: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
    assert_eq!(append::render(&params(1), &one, 1, 1, &mut refs).unwrap_err(), VecError::Shape);
    let two = [cst(&z), cst(&c)];
    assert_eq!(append::render(&params(1), &two, 3, 0, &mut refs).unwrap_err(), VecError::Shape);
}

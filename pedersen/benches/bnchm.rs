#[macro_use]
extern crate criterion;
extern crate pedersen_example;

use criterion::{Criterion, black_box};
use ark_crypto_primitives::{commitment::pedersen::Randomness};
//use ark_ed_on_bls12_377::*;
//use ark_ed_on_bls12_377::*;
//use ark_std::rand::SeedableRng;
use ark_ff::UniformRand;
use pedersen_example::*;

//use ark_bls12_377::Bls12_377;
//use ark_bls12_377::Bls12_377;

//use ark_groth16::*;
//use std::time::Instant;


criterion_group!(
	pederson_bench,
    groth_all_bench
);
criterion_main!(pederson_bench);

fn groth_all_bench(c1: &mut Criterion)
{
	println!("/////////////////////////////////");
    const REPETITION : usize = 50;
    #[cfg(feature="bls12_381")]
    println!("Benchmark for Pederson using Bls12_381 curve");
    #[cfg(feature="bls12_377")]
    println!("Benchmark for Pederson using Bls12_377 curve");
    #[cfg(feature="bn254c")]
    println!("Benchmark for Pederson using Bn254 curve");
	
	println!("Input size is: {:?} u8 elements", SIZEOFINPUT);
	println!("Output size is: {:?} Fp256<FrParameters> = {:?} bytes", 4, 4*2*8 );

    println!("/////////////////////////////////");

	let mut rng = ark_std::test_rng();

    // set up the proper inputs:
    
    // input is a 128 bytes of vector
	const INPTEXT:&str ="This is the input ...";
    const LEN : usize=  INPTEXT.len();
    let input = [
        INPTEXT.as_ref(),
        [0u8; SIZEOFINPUT - LEN].as_ref(),
    ]
   .concat();
    let inp =input;
	
	//let len = 128;
	//let input1 = vec![2u8; len];
	//println!("input1 = {:?}", input1.len());
	//  commit = Pedersen(input, param, open)
	let param = pedersen_setup(&[0u8; 32]);


	let open = Randomness::<JubJub>(Fr::rand(&mut rng));
	let commit = pedersen_commit(inp.as_ref(), &param, &open);
	//println!("commit = {:?}",commit);
	// build the input of the circuit
	let circuit = PedersenComCircuit {
		param: param.clone(),
		input: inp.to_vec(),
		open,
		commit,
	};
	//println!("com= {:?}",commit);
	// check the circuit is satisfied
	assert!(sanity_check1(circuit.clone()));
	let parameter2= param.clone();
	// generate the CRS for Groth16
	println!("start benchmarking grooth parameter generator");
	let mut bench_group = c1.benchmark_group("ZKP_param");
    bench_group.sample_size(REPETITION);
	let bench_str = format!("ZKP Parameter");
	bench_group.bench_function(bench_str, move |b| {
		b.iter(|| black_box(groth_param_gen(parameter2.clone())))
	});

	let zk_param = groth_param_gen(param);

	// generate the proof
	let zk_param2=zk_param.clone();
    let circuit2 = circuit.clone();
    //let elapse2 = start2.elapsed();
    //let start3 = Instant::now();
    
    println!("start benchmarking proof generator");
	//let mut bench_group = c.benchmark_group("proof");
    bench_group.sample_size(REPETITION);
	let bench_str = format!("ZKP proof");
	bench_group.bench_function(bench_str, move |b| {
		b.iter(|| black_box(groth_proof_gen(&zk_param2, circuit2.clone(), &black_box([32u8; 32]))))
	});
	let proof = groth_proof_gen(&zk_param, circuit.clone(), &[0u8; 32]);

	// verify the proof
	let proof2 = proof.clone();
    //let elapse3 = start3.elapsed();
    let zk_param3=zk_param.clone();
    let out2=commit.clone();
   
    //let start4 = Instant::now();

    println!("start benchmarking verification");
	//let mut bench_group = c.benchmark_group("verify");
    bench_group.sample_size(REPETITION);
	let bench_str = format!("ZKP Verification");
	bench_group.bench_function(bench_str, move |b| {
		b.iter(||     black_box(groth_verify(&zk_param3, &proof2, &out2))
    )
	});
	bench_group.finish();

	assert!(groth_verify(&zk_param, &proof, &commit));

//criterion_main!(proof_bench,verify_bench);
}
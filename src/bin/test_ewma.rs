#![allow(non_snake_case)]
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {

    let path = "keys";
    
    println!("loading LWE sk 0... \n");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    
    println!("loading LWE sk 1... \n");
    let sk1_LWE_path = format!("{}/sk1_LWE.json",path);
    let sk1 = LWESecretKey::load(&sk1_LWE_path).unwrap();    

    let m: Vec<f64> = vec![2.54]; // initial value for moving average process
    println!("ewma at t=0 {:?}\n", m);
    
    let mut x = 0.0; //: Vec<f64> = vec![0.0]; // initial value for data generating process
    println!("data at t=0 {:?}\n", x);
    
    // create an encoder
    let enc = Encoder::new(0., 10., 8, 1)?;

    let mut m0 = VectorLWE::encode_encrypt(&sk0, &m, &enc)?;  
    println!("ewma* {:?}", m0.decrypt_decode(&sk0).unwrap());
    m0.pp();

    println!("loading BSK 01... \n");
    let bsk01_path = format!("{}/bsk01_LWE.json",path);
    let bsk01 = LWEBSK::load(&bsk01_path);

    println!("loading KSK 10... \n");
    let ksk10_path = format!("{}/ksk10_LWE.json",path);
    let ksk10 = LWEKSK::load(&ksk10_path);    
    
    for i in 0..10 {
        println!("t={}, x={}\n", i, x);
        
        let x0 = VectorLWE::encode_encrypt(&sk0, &vec![x], &enc)?;  
        println!("data* {:?}", x0.decrypt_decode(&sk0).unwrap());
        x0.pp();
        
        let x1 = x0.bootstrap_nth_with_function(&bsk01, |x| 0.5 * x, &enc,0)?;
        x1.pp();
        let mut m1 = m0.bootstrap_nth_with_function(&bsk01, |x| 0.5 * x, &enc,0)?;
        m1.pp();
        
        m1.add_with_new_min_inplace(&x1, &vec![0.0])?; 
        println!("ewma* {:?}", m1.decrypt_decode(&sk1).unwrap());
        m1.pp();
        
        m0 = m1.keyswitch(&ksk10).unwrap();
        x = (x + 1.0)%10.;
    }
    
    Ok(())
}

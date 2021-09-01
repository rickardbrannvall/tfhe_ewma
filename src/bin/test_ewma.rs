#![allow(non_snake_case)]
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {

    let path = "device/keys80x1024_64";
    //let path = "device/keys80x1024_53";
    
    println!("loading LWE sk 0... \n");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    
    println!("loading LWE sk 1... \n");
    let sk1_LWE_path = format!("{}/sk1_LWE.json",path);
    let sk1 = LWESecretKey::load(&sk1_LWE_path).unwrap();    
    
    // create an encoder
    let enc = Encoder::new(0., 10., 6, 1)?;

    let m0: Vec<f64> = vec![2.54];
    println!("plaintext value {:?}\n", m0);
    
    let mut c0 = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    println!("encrypted value {:?}", c0.decrypt_decode(&sk0).unwrap());
    c0.pp();
    
    println!("loading BSK 0... \n");
    let bsk01_path = format!("{}/bsk01_LWE.json",path);
    let bsk01 = LWEBSK::load(&bsk01_path);
    
    println!("loading BSK 1... \n");
    let bsk10_path = format!("{}/bsk10_LWE.json",path);
    let bsk10 = LWEBSK::load(&bsk10_path);

    let enc_out = Encoder::new(0., 10., 6, 1)?;
    let c1 = c0.bootstrap_nth_with_function(&bsk01, |x| 1.0 * x, &enc_out,0)?;
    println!("bootstrap mul {:?}", c1.decrypt_decode(&sk1).unwrap());
    c1.pp();   

    let m1: Vec<f64> = vec![2.54];
    let x0 = VectorLWE::encode_encrypt(&sk0, &m1, &enc)?;
    let x1 = VectorLWE::encode_encrypt(&sk1, &m1, &enc)?;
    
    for i in 1..=10 {
        println!("round {}\n",i);
        
        let a1 = x0.bootstrap_nth_with_function(&bsk01, |x| 0.5 * x, &enc_out,0)?;
        let b1 = c0.bootstrap_nth_with_function(&bsk01, |x| 0.5 * x, &enc_out,0)?;
        let c1 = a1.add_with_new_min(&b1, &vec![0.0])?; 
        println!("average one {:?}", c1.decrypt_decode(&sk1).unwrap());
        a1.pp();   
        b1.pp();   
        c1.pp();   

        let a0 = x1.bootstrap_nth_with_function(&bsk10, |x| 0.5 * x, &enc_out,0)?;
        let b0 = c1.bootstrap_nth_with_function(&bsk10, |x| 0.5 * x, &enc_out,0)?;
        c0 = a0.add_with_new_min(&b0, &vec![0.0])?; 
        println!("average two {:?}", c0.decrypt_decode(&sk0).unwrap());
        a0.pp();   
        b0.pp();   
        c0.pp();     
    }
   
    
    Ok(())
}

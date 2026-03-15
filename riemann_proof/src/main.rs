use std::f64::consts::PI;
fn main() {
    let t_pow: f64 = 177.0; 
    let density = (t_pow * PI).sqrt();
    println!("\n--- [ رصد الكثافة المحورية ] ---");
    println!("Target Scale: 10^{}", t_pow);
    println!("Spiral Density: {:.15}", density);
    println!("-------------------------------");
    println!("جاهز لتطبيق تعميم تكامل الحصر.");
}

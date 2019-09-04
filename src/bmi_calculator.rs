fn bmi(weight: f64, height: f64) -> &'static str {
    let bmi: f64 = weight / height.powi(2);

    if bmi <= 18.5 {
        "Underweight"
    } else if bmi <= 25.0 {
        "Normal"
    } else if bmi <= 30.0 {
        "Overweight"
    } else {
        "Obese"
    }
}

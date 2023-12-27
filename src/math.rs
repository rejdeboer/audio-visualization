// Human ears are more sensitive to high frequencies
// That's why lower frequencies have a way higher magnitude in music
// With A-weighting we can achieve a more balanced frequency spectrum
// Source: https://en.wikipedia.org/wiki/A-weighting
pub fn get_a_weighting(f: f32) -> f32 {
    (f32::powi(12194., 2) * f.powi(4)) / (
        (f.powi(2) + f32::powi(20.6, 2))
            * f32::sqrt((f.powi(2) + f32::powi(107.7, 2)) * (f.powi(2) + f32::powi(737.9, 2)))
            * (f.powi(2) + f32::powi(12194., 2))
    )
}

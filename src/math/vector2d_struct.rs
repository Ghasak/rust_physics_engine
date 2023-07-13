#[allow(unused_imports)]
use core::fmt;
use num_traits::{Float, FromPrimitive, Num, Zero};
#[allow(unused_imports)]
use std::ops::{Add, Div, Mul, Neg, Sub};
#[allow(dead_code)]
#[allow(clippy::assign_op_pattern)]
#[derive(Debug, Clone, Copy)]
pub struct GVector2d<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
#[allow(clippy::assign_op_pattern)]
impl<T> GVector2d<T>
where
    T: Add<T, Output = T> + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn add(self, other: GVector2d<T>) -> Self
    where
        T: Add<T, Output = T> + Copy,
    {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn sub(self, other: GVector2d<T>) -> Self
    where
        T: Sub<T, Output = T> + Copy,
    {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    pub fn mul(self, other: GVector2d<T>) -> Self
    where
        T: Mul<T, Output = T> + Copy,
    {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    pub fn div(self, other: Self) -> Option<Self>
    where
        T: Div<T, Output = T> + PartialEq + Zero + Copy,
    {
        let zero = T::zero();
        if other.x != zero || other.y != zero {
            Some(Self {
                x: self.x / other.x,
                y: self.y / other.y,
            })
        } else {
            None
        }
    }

    /// Magnitude
    /// # Magnitude function
    /// ## Concept
    /// The following function can compute the magnitude of a vector
    /// which is the scalar of the vector based on phethogoris mathematical law
    /// $mag = \sqr((x_2 - x_1)^2 + (y_2 - y_1)^2)$
    /// The value is given in the format of f32
    pub fn magnitude(self) -> f32
    where
        T: Float + Mul<T, Output = T> + Copy,
    {
        let x_squared = self.x * self.x;
        let y_squared = self.y * self.y;
        (x_squared + y_squared).sqrt().to_f32().unwrap()
    }

    /// Normalize the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Float;
    /// use std::ops::Div;
    ///
    /// let v = GVector2d::new(3.0, 4.0);
    /// let normalized = v.normalize();
    /// println!("Normalized: {:?}", normalized);
    /// ```
    ///
    /// # Returns
    ///
    /// - If the magnitude of the vector is not zero, it returns a `Result` containing the normalized vector.
    /// - If the magnitude is zero, it returns an `Err` with an error message.
    pub fn normalize(&self) -> Result<Self, String>
    where
        T: Float + Div<T, Output = T> + Copy + FromPrimitive + Zero,
    {
        let m = self.magnitude();
        let zero = T::zero();
        if m != zero.to_f32().unwrap() {
            Ok(Self {
                x: self.x / T::from_f32(m).unwrap(),
                y: self.y / T::from_f32(m).unwrap(),
            })
        } else {
            Err("Cannot normalize a zero-length vector.".to_string())
        }
    }
    /// Rotate the vector by a given angle.
    ///
    /// The `rotate` method applies a rotation transformation to the vector
    /// by rotating its coordinates by the specified angle `theta`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::{Float, FromPrimitive};
    /// use std::ops::{Mul, Sub};
    ///
    /// let v = GVector2d::new(3.0, 4.0);
    /// let rotated = v.rotate(1.0); // Rotate by 1 radian
    /// println!("Rotated: {:?}", rotated);
    /// ```
    ///
    /// # Type Parameters
    ///
    /// - `N`: The type of the angle `theta`.
    ///
    /// # Parameters
    ///
    /// - `theta`: The angle by which to rotate the vector.
    ///
    /// # Returns
    ///
    /// The rotated vector with the coordinates adjusted based on the rotation angle.
    ///
    /// # Constraints
    ///
    /// - `T`: The type of the vector's coordinates, which must support multiplication and subtraction.
    /// - `N`: The type of the angle, which must implement the `Num`, `Float`, and `Sub` traits.
    ///
    pub fn rotate<N>(&self, theta: N) -> Self
    where
        T: Mul<N, Output = T> + Copy + Sub<Output = T>,
        N: Num + Float + Sub<Output = T> + Copy + FromPrimitive,
    {
        let cos_theta: N = theta.cos();
        let sin_theta: N = theta.sin();
        Self {
            x: self.x * cos_theta - self.y * sin_theta,
            y: self.x * sin_theta + self.y * cos_theta,
        }
    }
}


#[allow(dead_code)]
#[allow(clippy::assign_op_pattern)]
impl<T, U> GVector2d<T, U> {
    pub fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    /// Calculate the dot product between two vectors.
    ///
    /// The dot product, also known as the scalar product, is a measure of similarity between two vectors.
    /// It calculates the sum of the products of the corresponding components of the two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = GVector2d::new(3, 4);
    /// let w = GVector2d::new(2, 5);
    /// let dot_product = v.dot_product(&w);
    /// println!("Dot product: {}", dot_product);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `other`: The other vector to calculate the dot product with.
    ///
    /// # Returns
    ///
    /// The dot product of the two vectors, which is of type `T`.
    ///
    /// # Constraints
    ///
    /// - `T`: The type of the vector's coordinates, which must support multiplication and addition.
    /// - `U`: The type of the other vector's coordinates, which must support copying.
    ///
    /// # Edge Cases
    ///
    /// The dot product can be used to determine various properties of vectors:
    /// - If the dot product is zero, the vectors are orthogonal (perpendicular) to each other.
    /// - If the dot product is positive, the vectors have a similar direction.
    /// - If the dot product is negative, the vectors have an opposite direction.
    ///
    pub fn dot_product<V, W>(&self, other: GVector2d<V, W>) -> T
    where
        T: Mul<V, Output = T> + Add<Output = T> + Copy,
        U: Mul<W, Output = U> + Copy,
    {
        self.x * other.x + self.y * other.y
    }
}

// ----------------------------------------------------
//        Implemnetation for (+)(-)(/)(*) operators
// ----------------------------------------------------
// ---------- addition (+) -------------
impl<T> Add<GVector2d<T>> for GVector2d<T>
where
    T: Add<T> + Copy,
{
    type Output = GVector2d<T::Output>;
    fn add(self, other: GVector2d<T>) -> Self::Output {
        GVector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// ---------- subtarction (-) -------------
impl<T> Sub<GVector2d<T>> for GVector2d<T>
where
    T: Sub<T> + Copy,
{
    type Output = GVector2d<T::Output>;
    fn sub(self, other: GVector2d<T>) -> Self::Output {
        GVector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// ---------- multiplication (*) -------------
impl<T> Mul<GVector2d<T>> for GVector2d<T>
where
    T: Mul<T> + Copy,
{
    type Output = GVector2d<T::Output>;
    fn mul(self, other: GVector2d<T>) -> Self::Output {
        GVector2d {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

// ---------- division (/) -------------
impl<T> Div<GVector2d<T>> for GVector2d<T>
where
    T: Div<T, Output = T> + PartialEq + Zero + Copy,
{
    type Output = Option<GVector2d<T>>;

    fn div(self, other: GVector2d<T>) -> Self::Output {
        let zero = T::zero();
        if other.x != zero && other.y != zero {
            Some(GVector2d {
                x: self.x / other.x,
                y: self.y / other.y,
            })
        } else {
            None
        }
    }
}

// pub fn add_fn(&self, other: &GVector2d) -> Self {
//     Self {
//         x: self.x + other.x,
//         y: self.y + other.y,
//     }
// }

// pub fn sub_fn(&self, other: &GVector2d) -> Self {
//     Self {
//         x: (self.x - other.x),
//         y: (self.y - other.y),
//     }
// }
// pub fn magnitude(&self) -> f32 {
//     (self.x * self.x + self.y * self.y).sqrt()
// }

// pub fn normalize(&self) -> Self {
//     let m = self.magnitude();
//     Self {
//         x: self.x / m,
//         y: self.y / m,
//     }
// }
// pub fn rotate(&self, theta: f32) -> Self {
//     let cos_theta: f32 = theta.cos();
//     let sin_theta: f32 = theta.sin();
//     Self {
//         x: self.x * cos_theta - self.y * sin_theta,
//         y: self.x * sin_theta + self.y * cos_theta,
//     }
// }

// pub fn dot_product(&self, other: &GVector2d) -> f32 {
//     self.x * other.x + self.y * other.y
// }
// pub fn mult(&self, other: GVector2d) -> Self {
//     Self {
//         x: self.x * other.x,
//         y: self.y * other.y,
//     }
// }
// pub fn divide(&self, other: GVector2d) -> Self {
//     Self {
//         x: self.x / other.x,
//         y: self.y / other.y,
//     }
// }

// pub fn scale(&self, scale: f32) -> Self {
//     Self {
//         x: self.x * scale,
//         y: self.y * scale,
//     }
// }
// pub fn divide_by_scaler(&self, scaler: f32) -> Result<Self, String> {
//     if scaler == 0.0 {
//         let mut error_message = String::new();
//         error_message.push_str("[ERROR]:: Cannot divide GVector2d type by 0.0 ");
//         Err(error_message)
//     } else {
//         return Ok(Self {
//             x: self.x / scaler,
//             y: self.y / scaler,
//         });
//     }
// }

// pub fn projection_onto(&self, other: &GVector2d) -> Self {
//     let projection_dot = self.dot_product(&other);
//     let m = other.magnitude() * other.magnitude();
//     let temp = projection_dot / m;
//     Self {
//         x: other.x * temp,
//         y: other.y * temp,
//     }
// }
// pub fn orthogonal(&self, other: &GVector2d) -> Self {
//     let projection_vector = self.projection_onto(&other);
//     let orthogonal_vector = self.sub_fn(&projection_vector);
//     orthogonal_vector
// }

// -----------------------------------------------------------
#[test]
fn testing_vector() {
    let v1 = GVector2d::new(1.0, 0.0);
    let v2 = GVector2d::new(5.0, 7.3);

    println!("{v1:#?}");
    println!("{v2:#?}");
}
#[test]
fn testing_normalizing() {
    let v1 = GVector2d::new(1.0, 0.0);
    let v3 = GVector2d::normalize(&v1);
    println!("{v3:#?}");
    let m = v3.magnitude();
    println!("We are exepcting the normalized vector magnitude equal to 1 -> {m:#?}");
    assert_eq!(m, 1.0);
}

#[test]
fn testing_operations() {
    use std::f32::consts::FRAC_PI_2;
    let v1 = GVector2d::new(1.0, 0.0);
    let v2 = GVector2d::new(5.0, 7.3);
    let v4 = v1.add_fn(&v2);
    println!("We also can use the dot -> {v4:#?}");

    let v5 = v1.rotate(FRAC_PI_2);
    println!("{v5:#?}");
    assert_eq!(v5.x.round(), GVector2d::new(0.0, 1.0).x);
    assert_eq!(v5.y.round(), GVector2d::new(0.0, 1.0).y);
    let sin_x = FRAC_PI_2.sin();
    let cos_x = FRAC_PI_2.cos();
    println!("sin ->   {sin_x:#?}");
    println!("cos ->   {cos_x:#?}");
}
#[test]
fn testing_dot_product() {
    // About projection
    let v1 = GVector2d::new(3.0, 4.0);
    let v2 = GVector2d::new(5.0, 2.0);

    let v3 = v1.dot_product(&v2);
    let output_message = format!("projection v1 on v2 is -> {v3:#?}");
    println!("{output_message:#?}");
    let projection_v1_on_v2 = v1.projection_onto(&v2);
    let output_message = format!("projection v1 on v2 is -> {projection_v1_on_v2:#?}");
    println!("{output_message:#?}");
    let orthogonal_to_projection_v1_on_v2 = v1.orthogonal(&v2);
    let output_message =
        format!("projection v1 on v2 is -> {orthogonal_to_projection_v1_on_v2:#?}");
    println!("{output_message:#?}");
}

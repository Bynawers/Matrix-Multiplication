type Element = f64;
use rand::prelude::*;
use std::env;


#[derive(Debug, PartialEq)]
pub struct Matrix {
    n: usize,
    values: Vec<Element>,
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = Element;

    fn index(&self,(i,j):(usize,usize)) -> &Self::Output {
        &self.values[i*self.n+j]}
} 

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    
    fn index_mut(&mut self,(i,j):(usize,usize)) -> &mut Self::Output {
        &mut self.values[i*self.n+j]}
} 

impl Matrix {

    fn new(n1: usize, values: Vec<Element>) -> Self {
        if values.len() != n1*n1 { 
            panic!("Error!");
        }
        return Matrix {
            n: n1,
            values: values,
        }
    }
    fn id(n:usize) -> Self {
        let mut matrix = Matrix::new(n, vec![0.0;n*n]);

        for i in 0..n {
            matrix[(i,i)] = 1.0;
        }
        return matrix;
    }
    fn random(n:usize) -> Self {
        let mut valeurs:Vec<Element> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..n*n {
            valeurs.push(rng.gen_range(-1.0..1.0));
        }
        Matrix {
            n: n,
            values: valeurs,
        }   
    }

}
fn multiply(a: &Matrix, b: &Matrix) -> Matrix {

    if a.n != b.n {
        panic!("erreur! les matrices sont pas de la méme dimension");
    }
    let mut c = Matrix::new(a.n, vec![0.0; a.n*a.n]);

    for i in 0..a.n {
        for j in 0..a.n { 
            for k in 0..a.n{
                c[(i, j)] += a[(i, k)]*b[(k, j)];
            }
        }
    }
    c
}

fn block_multiply(a: &Matrix, b: &Matrix) -> Matrix {

    if a.n != b.n {
        panic!("erreur! les matrices sont pas de la méme dimension");
    }
    let mut c = Matrix::new(a.n, vec![0.0; a.n*a.n]);
    let nb = c.number_of_block();
    for jj in 0..nb{
        for kk in 0..nb{
            for i in 0..a.n{
                for j in jj*Matrix::BLOCK..(jj+1)*Matrix::BLOCK{
                    for k in kk*Matrix::BLOCK..(kk+1)*Matrix::BLOCK{
                        c[(i,j)] += a[(i,k)]*b[(k,j)];
                    }
                }
            }
        }
    }
}

fn main() { 
    let args: Vec<_> = env::args().collect();
    let mut size: usize = 0;

    if args.len() == 2 {
        size = args[1].parse().unwrap();
    }
    if args.len() == 3 {
        size = args[2].parse().unwrap();
    }

    let matrix_a = Matrix::random(size);
    let matrix_b = Matrix::random(size);

    let _mult_matrix = multiply(&matrix_a, &matrix_b);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn Exemple() {
        let matrix = Matrix::new(2, vec![1.0,2.2,5.0,3.6] );
        assert_eq!(2, matrix.n);
        assert_eq!(matrix.values, vec![1.0,2.2,5.0,3.6]);
    }

    #[test]
    fn Index() {
        let mut m = Matrix::new(2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(1, 0)], 3.0);

        m[(1,0)] = 5.0;
        assert_eq!(m[(1, 0)], 5.0);
    }

    #[test]
    fn Identite() {
        let matrix = Matrix::id(4);
        assert_eq!(4, matrix.n);
        assert_eq!(matrix.values, vec![1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0]);
    }
}


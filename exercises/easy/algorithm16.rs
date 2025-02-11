/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    //  Implement the logic to rotate the matrix 90 degrees in place
    let old_row_num = matrix.len();
    let old_col_num = matrix.get(0).unwrap().len();
    let mut old_row_num_t = old_row_num;
    let new_row_num = old_col_num;
    let new_col_num = old_row_num;
    
    // 补齐新矩阵缺失的行
    while old_row_num_t < new_row_num {
        matrix.insert(old_row_num_t, vec![0;new_col_num]);
        old_row_num_t += 1;
    }
    // 补全新矩阵缺失的列
    for i in matrix.iter_mut() {
        let mut old_col_num_t = i.len();
        while old_col_num_t < new_col_num {
            i.push(0);
            old_col_num_t += 1;
        }
    }
    struct Tempkeep {
        row: usize,
        col: usize,
        val: i32,
    }

    let mut temp_keep_vec: Vec<Tempkeep> = Vec::new();

    // 旋转90度的操作
    {
        let mut t_row = 0;
        while t_row < old_row_num {
            let mut t_col = 0;
            let v_c = matrix.get_mut(t_row).unwrap().clone();
            while t_col < old_col_num {
                // 旋转90度
                let new_col_t = old_row_num - t_row - 1;
                let new_row_t = t_col;
                let val = v_c.get(t_col).unwrap();
                if new_row_t <= t_row {
                    // 直接放置
                    let vv = matrix.get_mut(new_row_t).unwrap().get_mut(new_col_t).unwrap();
                    *vv = *val;
                }else {
                    // 放到temp keep中
                    let temp_keep = Tempkeep {
                        row: new_row_t,
                        col: new_col_t,
                        val: *val,
                    };
                    temp_keep_vec.push(temp_keep);
                }

                t_col += 1;
            }
            t_row += 1;
        }
    }
    // 补全temp keep
    for i in &temp_keep_vec {
        let Tempkeep { row, col, val } = *i;
        *matrix.get_mut(row).unwrap().get_mut(col).unwrap() = val;
    }
    {
        // 如果新的列小于旧的列，进行收缩
        let mut new_col_num_t = new_col_num;
        while new_col_num_t < old_col_num {
            let mut i = 0;
            while i < matrix.len() {
                let mut aa = matrix.get_mut(i).unwrap();
                aa.pop();
                i += 1;
            }
            new_col_num_t += 1;
        }
    }
   
    {
        // 如果新的行小于旧的行，进行收缩
        let mut new_row_num_t = new_row_num;
        while new_row_num_t < old_row_num {
            matrix.pop();
            new_row_num_t += 1;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}

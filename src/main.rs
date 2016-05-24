/*//Reinier Maas reiniermaas@hotmail.com
//Rosetta code reduced row echelon form RREF in Rust
//http://rosettacode.org/wiki/Reduced_row_echelon_form
//function ToReducedRowEchelonForm(Matrix M) is
//    lead := 0
//    rowCount := the number of rows in M
//    columnCount := the number of columns in M
//    for 0 ≤ r < rowCount do
//        if columnCount ≤ lead then
//            stop
//        end if
//        i = r
//        while M[i, lead] = 0 do
//            i = i + 1
//            if rowCount = i then
//                i = r
//                lead = lead + 1
//                if columnCount = lead then
//                    stop
//                end if
//            end if
//        end while
//        Swap rows i and r
//        If M[r, lead] is not 0 divide row r by M[r, lead]
//        for 0 ≤ i < rowCount do
//            if i ≠ r do
//                Subtract M[i, lead] multiplied by row r from row i
//            end if
//        end for
//        lead = lead + 1
//    end for
//end function
//Test matrix:
//1   2   -1   -4
//2   3   -1   -11
//-2   0   -3   22
//Result matrix:
//1   0   0   -8
//0   1   0   1
//0   0   1   -2
use std::io;
use std::io::BufferedReader;
fn main() {
    println!("This reduces an Matrix to 'Reduced Row Echelon Form'.");
	println!("Input like:");
	println!("0 1 2 3 ...");
	println!("4 5 6 7 ...");
	println!("...");
	println!("End");
		
	let mut reader = BufferedReader::new(io::stdin());
	let mut matrix : Vec<Vec<i32>> = reader.lines()
			.map(|l| l.unwrap())
			.take_while(|l| !l.trim().is_empty())
			.map(|l: String| -> Vec<i32> l.words().filter_map(|s| s.parse()).collect())
			.collect();
	let row_count = matrix.len();
	let column_count = matrix[0].len();
	println!("{}", matrix_to_str(to_rref(matrix)));
}
fn matrix_to_str(a : Vec<Vec<i32>>) -> String {
    let mut b : Vec<Vec<String>> = 	vec![vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
										 vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
										 vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()]];
	let mut max = 0;
	for r in range(0, row_count) {
		for c in range(0, column_count) {
			let string = format!("{}",a[r][c]);
			b[r][c] = string;
			if max < b[r][c].len() {
				max = b[r][c].len();
			}
		}
	}
	max += 1;
	let mut array_str : String = "".to_string();
	for r in b.iter() {
		let mut line = "
		[".to_string();
		for i in r.iter() {
			for _ in range(i.len(), max) {
				line.push_str(" ");
			}
			line.push_str(i.as_slice());
		}
		line.push_str(" ]
		");
		array_str.push_str(line.as_slice());
	}
	array_str
}
fn to_rref(a : Vec<Vec<i32>>) -> Vec<Vec<i32>> {
	let row_count = a.len();
	let column_count = a[0].len();
	let mut b = a;
	let mut lead = 0;
	for r in range(0, row_count) {
		let mut i = r;
		if column_count <= lead {
			break;
		}
		while b[i][lead] == 0 {
			i += 1;
			if row_count == i {
				i = r;
				lead += 1;
				if column_count == lead {
					break;
				}
			}
		}
		//swap rows i and r
		b.as_mut_slice().swap(i,r);
		//swap(i, r, &mut b);
		if b[r][lead] != 0 {
			let divide = b[r][lead];
			for c in range (0, column_count) {
				b[r][c] = b[r][c] / divide;
			}
		}
		for i in range(0, row_count){
			if i != r {
				let multiplied = b[i][lead];
				for c in range (0, column_count) {
					b[i][c] = b[i][c] - (multiplied * b[r][c]);
				}
			}
		}
		lead += 1;
	}
	b
}

#[test]
fn basic_test() {
    let input = [[ 1, 2, -1,  -4],
                 [ 2, 3, -1, -11],
                 [-2, 0, -3,  22]];
    let expected_output = [[1, 0, 0, -8],
                           [0, 1, 0,  1],
                           [0, 0, 1, -2]];
    let output = to_rref(input);
    assert_eq!(expected_output, output);
}
*/
/*//http://rosettacode.org/wiki/Reduced_row_echelon_forms

use std::io;
fn main() {

    let mut reader = io::stdin();
    let matrix: Vec<Vec<i32>> = reader.lock().lines()
            .map(|l| l.unwrap())
            .take_while(|l| !l.trim().is_empty())
            .map(|l| l.words().filter_map(|s| s.parse()).collect())
            .collect();
    let matrix = [[1, 2, 3, 4]
                  [2, 3, 4, 5]
                  [5, 2, 3, 0]
                  [1, 2, 7, 8]];
    let rref = to_rref(&matrix);
    println!("{}", vec2d_to_str(rref));
}
fn vec2d_to_str(a : Vec<Vec<i32>>, row_count : usize, column_count : usize) -> String {
    let mut hewaniit : Vec<Vec<String>> =  vec![vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
                                         vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
                                         vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()]];
    let mut max = 0;
    for r in range(0, row_count) {
        for c in range(0, column_count) {
            let string = format!("{}",a[r][c]);
            b[r][c] = string;
            if max < b[r][c].len() {
                max = b[r][c].len();
            }
        }
    }
    max += 1;
    let mut array_str : String = "".to_string();
    for r in b.iter() {
        let mut line = "
        [".to_string();
        for i in r.iter() {
            for _ in range(i.len(), max) {
                line.push_str(" ");
            }
            line.push_str(i.as_slice());
        }
        line.push_str(" ]
        ");
        array_str.push_str(line.as_slice());
    }
    array_str
}
fn to_rref(a: &[[i32]]) -> Vec<Vec<i32>> {
    let row_count = a.len();
    let column_count = a[0].len();
    let mut b = a;
    let mut lead = 0;
    for r in range(0, row_count) {
        let mut i = r;
        if column_count <= lead {
            break;
        }
        while b[i][lead] == 0 {
            i += 1;
            if row_count == i {
                i = r;
                lead += 1;
                if column_count == lead {
                    break;
                }
            }
        }
        //swap rows i and r
        b.as_mut_slice().swap(i,r);
        //swap(i, r, &mut b);
        if b[r][lead] != 0 {
            let divide = b[r][lead];
            for c in range (0, column_count) {
                b[r][c] = b[r][c] / divide;
            }
        }
        for i in range(0, row_count){
            if i != r {
                let multiplied = b[i][lead];
                for c in range (0, column_count) {
                    b[i][c] = b[i][c] - (multiplied * b[r][c]);
                }
            }
        }
        lead += 1;
    }
    b
}

#[test]
fn basic_test() {
    let input = [[ 1, 2, -1,  -4],
                 [ 2, 3, -1, -11],
                 [-2, 0, -3,  22]];
    let expected_output = [[1, 0, 0, -8],
                           [0, 1, 0,  1],
                           [0, 0, 1, -2]];
    let output = to_rref(input);
    assert_eq!(expected_output, output);
}*/

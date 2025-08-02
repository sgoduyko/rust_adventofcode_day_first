

fn calc_total_distance(
       first_arr: &mut [i32],
       second_arr: &mut [i32],
) -> i32 {
       // problems of that approach
       // 1) complexity is growing when grow len of array
       // 2) we sort the arrays, but we must just calculate total distance and nothing more

       if first_arr.len() != second_arr.len() {
              panic!("length of arrays must to be equal.");
       };
       let mut total_distance: i32 = 0;
       for i in 0..first_arr.len() {
              
              if i + 1 > first_arr.len() {
                     break
              }

              for k in i+1..first_arr.len() {
                     if first_arr[k] < first_arr[i] {
                            let temp = first_arr[i];
                            first_arr[i] = first_arr[k];
                            first_arr[k] = temp;
                     }
                     
                     if second_arr[k] < second_arr[i] {
                            let temp = second_arr[i];
                            second_arr[i] = second_arr[k];
                            second_arr[k] = temp;
                     }
              }
              let diff = (first_arr[i] - second_arr[i]).abs(); 
              total_distance += diff
       };
       total_distance
}
fn main() {
       let mut first_arr_loc_ids: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
       let mut second_arr_loc_ids: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
       
       let total_distance: i32 = calc_total_distance(&mut first_arr_loc_ids, &mut second_arr_loc_ids);
       println!("total_distance={total_distance}");
}

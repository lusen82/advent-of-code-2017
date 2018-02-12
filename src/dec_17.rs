



use super::print_utils;

fn day17() {
    let mut pos = 1;
    let step = 359;
    let mut buffer: Vec<usize> = vec![0, 1];
    for i in 2..50000000{
        let next_pos = get_next_pos_len(step, pos, i);//&buffer);
        if next_pos == 0 {
            buffer.push(i);
        }
        pos = next_pos + 1;
    }
    println!("Tal {}", &buffer[1]);
   print_utils::print_vector_u(&buffer);
}

fn get_next_pos_len(step: usize, pos: usize, buffer_len: usize) -> usize{

    let rest_step = step % buffer_len;
    if pos == (buffer_len - 1) {
        return rest_step -1;
    }
    if step == buffer_len {
        return pos;
    }

    let mut n = pos + rest_step;
    if n < buffer_len
        {
            return n;

        }
    if n >buffer_len
        {
            return n - buffer_len;

        }
    if n == (buffer_len) {

        // println!("1n was merely too high {} and pos {} length {} step_sozze{}", n, pos, buffer.len(), rest_step);
        n = rest_step - (buffer_len - pos);
    }
    /*    if n >= (buffer.len()) {

            println!("1n was too high {} and pos {} length {} step_sozze{}", n, pos, buffer.len(), rest_step);
            n = n % buffer.len() + pos;
            if (n >= buffer.len()) {
                n = n % buffer.len();
            }
        }
        println!("n {} and pos{} length {}", n, pos, buffer.len());*/

    return n;
}
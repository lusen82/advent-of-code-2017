#[cfg(test)]
mod main;

mod tests {
    use super::*;

//    #[test]
    //    pub fn test_day6()
    //    {
    //        let inp: &'static str = "0 2 7 0";
    //
    //        main::day_6_b_calc(&inp);
    //
    ////        let vec_ints: Vec<i32> = vec![0, 2, 7, 4];
    ////
    ////        let add_on_others: i32 = 2;
    ////        let updated: Vec<i32> = main::get_vec_add_to_all(&vec_ints, &add_on_others);
    ////
    ////        assert_eq!(i32::from(updated[0]), 2);
    ////        assert_eq!(i32::from(updated[1]), 4);
    ////        assert_eq!(i32::from(updated[2]), 9);
    ////        assert_eq!(i32::from(updated[3]), 6);
    ////
    //
    //
    // }
        #[test]
        pub fn test_day_10()
        {
            let first = main::take_first_reversed_sublist();
            println!("Length: {}", first.len());
            assert_eq!(255 as usize, first.len());
            assert_eq!(87 as i32, *first.first().unwrap());
            let vec = vec![1,2,3,4,5,6,7,8,9];
            let res = main::get_sub_split(vec, 6, 7);
            println!("{}", res[0]);

            let vec = vec![2,1,0,3,4];
            let res = main::get_sub_split(vec, 3, 4);
            println!("{}", res[0]);
            assert_eq!(vec![4,3,0,1,2], res);

            let vec = vec![1,2,3,4,5,6,7,8,9];
            let res = main::get_sub_split(vec, 6, 7);
            println!("{}", res[0]);
     }

//    #[test]
//    pub fn test_day_9()
//    {
//        let stripped: String = main::remove_ignored_characters("{{<!>},{<!>},{<!>},{<a>}}");
//        assert_eq!(stripped, "{{<!},{<!},{<!},{<a>}}");
//
//        let stripped: String = main::remove_ignored_characters("{{<!!>}");
//        assert_eq!(stripped, "{{<!>}");
//        let stripped: String = main::remove_ignored_characters("{{<!!!>}");
//        assert_eq!(stripped, "{{<!!}");
//
//        let removed_garbage: String = main::remove_included_garbage("<random characters>".to_string());
//        assert_eq!(removed_garbage, "");
//
//        let removed_garbage: String = main::remove_included_garbage("<ran>dom characters>".to_string());
//        assert_eq!(removed_garbage, "dom characters>");
//
//        let removed_garbage: String = main::remove_included_garbage("{{<!},{<!},{<!},{<a>}}".to_string());
//        assert_eq!(removed_garbage, "{{}}");
//
//        let removed_garbage: String = main::remove_all_garbage( "gfag<{oi!a,<{i<a>ghjkghg<{oi!a,<{i<a><{oi!a>,<{i<a>j".to_string());
//        assert_eq!(removed_garbage, "gfagghjkghg,j");
//
//        let find_groups = main::find_groups("{{{},{},{{}}}}".to_string());
//        assert_eq!(find_groups, 16);
//        let find_groups = main::find_groups("{{},{}}".to_string());
//        assert_eq!(find_groups, 5);
//        let find_groups = main::find_groups("{{{}}}".to_string());
//        assert_eq!(find_groups, 6);
//
//        let removed_garbage = main::remove_included_garbage("{<{},{},{{}}>}".to_string());
//        let find_groups = main::find_groups(removed_garbage);
//        assert_eq!(find_groups, 1);
//        let removed_garbage = main::remove_included_garbage("{<a>,<a>,<a>,<a>}".to_string());
//        let find_groups = main::find_groups(removed_garbage);
//        assert_eq!(find_groups, 1);
//        let removed_garbage = main::remove_included_garbage("{{<a>},{<a>},{<a>},{<a>}}".to_string());
//        let find_groups = main::find_groups(removed_garbage);
//        assert_eq!(find_groups, 9);
//
//        let stripped: String = main::remove_ignored_characters("{{<!>},{<!>},{<!>},{<a>}}");
//        let removed_garbage = main::remove_included_garbage(stripped);
//        let find_groups = main::find_groups(removed_garbage);
//        assert_eq!(find_groups, 3);
//
//    }


//    #[test]
//        pub fn test_day7()
//        {
//            main::day_7();
//    //
//    //        let vec_ints: Vec<i32> = inp.iter().map(|s| s.parse::<i32>().unwrap()).collect();
//    //
//    //               let res = main::re_calc_ints(vec_ints);
//        }
/*
    #[test]
    pub fn test_day1()
    {
        main::day1();
        assert_eq!("world", "world");
    }*/
}

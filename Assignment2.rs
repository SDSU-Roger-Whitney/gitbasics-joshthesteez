/******************************************************************************************************
Programmer: Josh Brown | 822455771
CS 420

9/18/2022 ~~ DUE DATE
~~ Assignment 2
******************************************************************************************************/

// ~~ Part 1
/******************************************************************************************************
 * 
(20 points) Create a public struct called StudentGrades. The struct contains a name and a
vector of grades. The StudentGrades struct has at least two methods (associated functions): average 
and grade. 

The average method returns the average score for that student.

The grade method returns the student's letter grade based on the scale below.

The StudentGrades should not lose any information 
when calling either average or grade.
 *
******************************************************************************************************/
//use std::str::FromStr;

#[derive(Clone, Default, Debug)]
struct StudentGrades { student_name: String, grade_vec: Vec<f64> }

trait FuncsForSG {
    fn average (&self) -> f64;
    fn grade (&self) -> char;
    // fn get_name(&self) -> &String;
    fn builder(name: String, grades: Vec<f64>) -> StudentGrades;
}

impl FuncsForSG for StudentGrades {

    fn average (&self) -> f64 {
        
        let mut n = 0.0;
        let mut avg = 0.0;

        // searches through grade_vec 
        // avg holds sum all of the assignments
        // n keeps track of how many assignments there are
        for grade in self.grade_vec.iter() {
            avg += grade; 
            n += 1.0;
        }

        avg = avg / n;

        if avg > 100.0 {
            eprintln!("ERROR: average grade > 100%");
            return -1.0
        }
        else {
            return avg
        }
    }

    fn grade (&self) -> char {
        let grade: f64 = self.average();
        if grade <= 59.0 {return 'F'}
        else if grade > 59.0 && grade <= 69.0  { return 'D'}
        else if grade > 69.0 && grade <= 79.0  { return 'C'}
        else if grade > 79.0 && grade <= 89.0  { return 'B'}
        else if grade > 89.0 && grade <= 100.0 { return 'A'}
        else {
            eprintln!("ERROR: grades entered in wrong format");
            return 'X'
        }
    }

    // fn get_name(&self) -> &String { return &self.student_name }

    fn builder(name: String, grades: Vec<f64>) -> StudentGrades {
        return StudentGrades { student_name: name, grade_vec: grades };
    }

}


// ~~ Part 2
/******************************************************************************************************
 * 
~~ 2) (20 points) Create a public struct called CourseGrades. The struct has a type-associated function 
from_ﬁle(ﬁle_path:String). The argument to the method is a path to a ﬁle. The method reads the ﬁle. 
For each line in the ﬁle, it creates a StudentGrades object. Each line of the ﬁle is in the format:

name, grade1, grade2, ..., gradeN

That is, each line starts with a name, followed by a comma, which is followed by optional spaces, which 
are followed by a numerical score. Each score represents a grade event, that is, an assignment or exam. 
Grades are separated by a comma followed by optional spaces. Here is a sample ﬁle:

roger, 100.0, 80.0, 80
pete, 70, 75.5,73

The struct has at least two methods: (associated functions): average and student. 

With the input N an integer, the average method returns the average score of all students on the N'th grade event. 
The ﬁrst grade event is indexed by 1, not 0. average returns an Option, as the input N could be out of range. 

student has one parameter: a string which is a student's name. It returns an Option. If the name is found 
in the course, the StudentGrades is returned. Neither method should be destructive. You are not to use a 
CSV crate or libraries.
 *
******************************************************************************************************/
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use std::num::NonZeroI128;
// use std::ops::Add;

struct CourseGrades { student_vec: Vec<StudentGrades> }

trait FuncsForCG {
    fn average(&self, N: u32, students_list: &Vec<StudentGrades>) -> Option<f64>;
    fn student(&self, name: String) -> Option<StudentGrades>;
    fn from_file(&self, file_path: String) -> CourseGrades;
}

impl FuncsForCG for CourseGrades {

    fn average(&self, N: u32, students_list: &Vec<StudentGrades>) -> Option<f64> {
        let mut avg = 0.0;
        let mut num_students = 0.0;
        // outer loop
        // iterates through all students in student_vec
        for student in students_list.iter() {
            // inner loop
            // must break if a N is out of range
            // computes the average of all graded assignments
            if N < 1 {
                return None;
            }    
            else if N as usize > student.grade_vec.len() {
                return None;
            }
            else {
                let mut student_grade = student.grade_vec[N as usize];
                // println!("TEST StudentGrade value: {}", student_grade); ** USED FOR TESTING
                avg = avg + student_grade;
            }
            num_students += 1.0;
            // println!("TEST num_students value: {}", num_students); ** USED FOR TESTING
        }
        return Some(avg / num_students)
    }

    fn student(&self, name: String) -> Option<StudentGrades> {
        let mut found_student = StudentGrades::default();
        for student in &self.student_vec {
            if student.student_name == name {
                found_student = student.clone();
            }
        }
        if found_student.student_name != name {
            return None
        }
        else {
            return Some(found_student)
        }
    }


    // THIS IS UNFINISHED
    // I tried to make it work but I ran into borrowing errors, feel free to uncomment and test
/* 
    fn from_file(&self, file_path: String) -> CourseGrades {
        let file = File::open(file_path).expect("File not found."); //opens the file
        let buf_reader = BufReader::new(file); // allows us to scan each line of the file

        let mut SG_vec: Vec<f64> = [].to_vec(); // holds the float values from each line
        let mut CG_return: CourseGrades;

        // outer loop will read each line of the file and split by comma
        for line in buf_reader.lines() {
            let mut student_from_file: StudentGrades; // StudentGrades object to send to student_vec of new CourseGrades object

            let split_line = line.unwrap();
            let  mut collection_vec: Vec<&str> = split_line.trim().split(",").collect(); // parses lines separated by comma and cleans whitespace
            // strip the name from the vector of strings
            let name  = collection_vec[0];
            println!("NAME: {}", name);
            collection_vec.remove(0); // removes name from vector, now  the line will contain grade values as &str objects
            //SG_vec.parse().unwrap();

            // sends grades to SG_vec
            for element in collection_vec.iter() {
                let mut float: f64 = element.parse::<f64>().unwrap();
                SG_vec.push(float);
            
            }
            student_from_file = StudentGrades::builder(name.to_owned(), SG_vec); // now we build SG object to be pushed into CG vector
            CG_return.student_vec.push(student_from_file);

        }
        return CG_return
    }
*/


}

// ~~ Part 3
/******************************************************************************************************
 * 
~~ 3) Create a public struct called CourseSchedule. The struct has a type-associated function 
from_ﬁle(ﬁle_path:String). The ﬁle_path is a path to a .xls ﬁle of the SDSU schedule. One is downloadable 
from the same web page as this document. The struct has at least one method, courses_at(time: &str, days: &str). 
The time is given in 24-hour format—for example, 1800 and 815. Days are limited to M, T, W, TH, MWF, MW, and TTH. 
The method returns a vector of Course instances for courses being taught at the day and time indicated. 
The Struct Course has at least the ﬁelds: catalog_number, title, and instructor. All are string types. You can use
a CSV library on problem 3 if you like.
 *
*******************************************************************************************************/
// use office::{Excel, Range, DataType};
struct CourseSchedule {}

// ~~ Part 4 UNIT TEST
// *****************************************************************************************************

fn main() {

    println!("UNIT TEST\n");

    // PART 1 TEST (Struct: StudentGrades, funcs: average(), grade())
    println!("******************************\n");

    println!("Part 1: StudentGrades struct\n");

    println!("Creating StudentGrades object... ");
    let student1 = StudentGrades { student_name: "Josh".to_owned(), 
                                                  grade_vec: [70.0, 57.0, 93.0, 1.0, 74.0, 98.0].to_vec() };

    let student2 = StudentGrades { student_name: "David".to_owned(), 
                                                  grade_vec: [68.0, 15.0, 100.0, 75.0, 81.0, 63.0].to_vec() };

    let student3 = StudentGrades { student_name: "Mike".to_owned(), 
                                                  grade_vec: [90.0, 92.0, 95.0, 92.0, 100.0].to_vec() };

    println!("Testing average() method...");
    let student1_test_avg = StudentGrades::average(&student1);
    println!("average grade is: {}", student1_test_avg);

    println!("Testing grade() method...");
    let student1_test_grade = StudentGrades::grade(&student1);
    println!("letter grade is: {}", student1_test_grade);

    println!("Part 1: SUCCESS\n");
    println!("******************************\n");

    // PART 2 TEST (struct: CourseGrades, methods: from_file, average, student)
    println!("Part 2: CourseGrades struct\n");
    println!("Creating CourseGrades object... ");

    let grades1 = CourseGrades { student_vec: [student1, student2, student3].to_vec() };

    println!("Testing from_file() method...");
    let path: String = "C:\\Users\\jbeas\\Desktop\\Coding\\Rust\\part2test_data.txt".to_owned();
    grades1.from_file(path);

    println!("Testing average() method...");

    let avg_CG = grades1.average(3, &grades1.student_vec);
    println!("Average of 1st assignment {:?}", avg_CG);

    println!("Testing student() method...");
    let student_method_test = grades1.student("Mike".to_owned());
    println!("Expecting StudentGrades object \"Mike\", Output: {:?}", student_method_test);

}
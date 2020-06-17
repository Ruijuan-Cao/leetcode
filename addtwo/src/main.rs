/*
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example:

Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8
Explanation: 342 + 465 = 807.
*/

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
/*
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

    }
}
*/

/* c code - the first 10000 prime
#include <stdio.h>
#define MAXNUM 10000

void func()
{
    register int i, j;
    int cnt;
    int flag;
    int findvalue[MAXNUM] ={2};

    for(cnt = 1, i = 3; cnt < MAXNUM; i += 2)
    {
        for(j = 0; flag = ((findvalue[j] * findvalue[j]) <= i); j ++) {
            if(!(i % findvalue[j]))break;
        }
        if(!flag)findvalue[cnt ++] = i;
    }

    printf("%d\n", findvalue[MAXNUM - 1]);
}

int main()
{
    func();
    return 0;
}
*/

const MAXNUM: usize = 10000;
//get the first 10000 prime
pub fn get_10000_primes() {
    let mut flag = false;
    let mut value = vec![2; MAXNUM];
    let mut count = 1;
    let mut i = 3;
    loop {
        let mut j = 0;
        loop {
            flag = (value[j] * value[j] > i);
            if (i % value[j] == 0) || flag {
                break;
            }
            j += 1;
        }

        if flag {
            value[count] = i;

            count += 1;
        }
        if count == MAXNUM {
            break;
        }
        i += 2;
    }
    println!("primes:{:?}", value);
}

fn main() {
    println!("Hello, world!");
    get_10000_primes();
}

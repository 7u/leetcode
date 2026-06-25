use std::fmt::Display;

fn main() {
    // Input: l1 = [2,4,3], l2 = [5,6,4]
    // Output: [7,0,8]
    // 2->4->3 , 5->6->4
    // 7->0->8
    // Explanation: 342 + 465 = 807.

    let mut l1 = ListNode::build(&[2, 4, 3]);
    let mut l2 = ListNode::build(&[5, 6, 4]);

    // let mut l1 = ListNode::build(&[0]);
    // let mut l2 = ListNode::build(&[0]);

    // let mut l1 = ListNode::build(&[9, 9, 9, 9, 9, 9, 9]);
    // let mut l2 = ListNode::build(&[9, 9, 9, 9]);

    // let mut l1 = ListNode::build(&[5]);
    // let mut l2 = ListNode::build(&[5]);

    println!(
        "Input: l1={}, l2={}",
        l1.as_ref().unwrap(),
        l2.as_ref().unwrap()
    );

    if let Some(res) = Solution::add_two_numbers(l1, l2) {
        println!("Output: {res}");
    }
}

// Definition for singly-linked list.
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

    fn build(arr: &[i32]) -> Option<Box<Self>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut (dummy.next);
        for &i in arr {
            *tail = Some(Box::new(ListNode::new(i)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        dummy.next
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}", self.val)?;
        let mut tail = &self.next;
        while let Some(t) = tail {
            write!(f, ",{}", t.val)?;
            tail = &t.next;
        }
        write!(f, "]")
    }
}

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let (mut d, mut s, mut tens) = (0, 0, 0);

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut (dummy.next);

        loop {
            match (l1.take(), l2.take()) {
                (Some(l1_p), Some(l2_p)) => {
                    s = l1_p.val + l2_p.val + tens;
                    d = s % 10;
                    tens = s / 10;

                    *tail = Some(Box::new(ListNode::new(d)));
                    tail = &mut tail.as_mut().unwrap().next;

                    l1 = l1_p.next;
                    l2 = l2_p.next;
                }
                (Some(l1_p), None) => {
                    l1 = Some(l1_p);
                    break;
                }
                (None, Some(l2_p)) => {
                    l2 = Some(l2_p);
                    break;
                }
                _ => break,
            }
        }

        if l1.is_some() {
            (tail, tens) = Self::add_number(tail, l1, tens);
        }

        if l2.is_some() {
            (tail, tens) = Self::add_number(tail, l2, tens);
        }

        if tens != 0 {
            *tail = Some(Box::new(ListNode::new(tens)));
        }

        dummy.next
    }

    fn add_number(
        res: &mut Option<Box<ListNode>>,
        l: Option<Box<ListNode>>,
        tens: i32,
    ) -> (&mut Option<Box<ListNode>>, i32) {
        let (mut s, mut d) = (0, 0);
        let (mut l, mut tens) = (l, tens);

        let mut tail = res;

        while let Some(lp) = l {
            s = tens + lp.val;
            d = s % 10;
            tens = s / 10;

            *tail = Some(Box::new(ListNode::new(d)));
            tail = &mut tail.as_mut().unwrap().next;

            l = lp.next;
        }
        (tail, tens)
    }
}

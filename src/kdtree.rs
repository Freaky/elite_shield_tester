/* Rust port of  Jamie "Entity" van den Berge's Python kdtree implementation.

Ported to Rust by Thomas Hurst.

Awful hack warning: this abuses the last value of the points list as an ad-hoc
object identifier.  A better fix is forthcoming.
*/

#[derive(Debug, Default)]
pub struct KDTreeNode {
    potency: f64,
    location: Vec<f64>,
    left: Option<Box<KDTreeNode>>,
    right: Option<Box<KDTreeNode>>,
}

impl KDTreeNode {
    pub fn from_points(point_list: &mut [Vec<f64>]) -> Option<Box<KDTreeNode>> {
        KDTreeNode::import_depth(point_list, 0)
    }

    fn import_depth(point_list: &mut [Vec<f64>], depth: usize) -> Option<Box<KDTreeNode>> {
        if point_list.is_empty() {
            return None;
        }

        // Select axis based on depth so that axis cycles through all valid values
        let k = point_list[0].len() - 1; // assumes all points have the same dimension
        let axis = depth % k;

        // Sort point list and choose median as pivot element
        point_list.sort_by(|a, b| a[axis].partial_cmp(&b[axis]).unwrap());

        let median = point_list.len() / 2; // choose median

        let potency = point_list.last().expect("last index")[axis];
        let location = point_list[median].clone();
        let (mut left, right) = point_list.split_at_mut(median);
        let (_, mut right) = right.split_at_mut(1);

        let node = KDTreeNode {
            potency,
            location,
            left: KDTreeNode::import_depth(&mut left, depth + 1),
            right: KDTreeNode::import_depth(&mut right, depth + 1),
        };

        Some(Box::new(node))
    }

    pub fn dominates(&self, point: &[f64]) -> bool {
        self.dominates_axis(point, 0)
    }

    fn dominates_axis(&self, point: &[f64], axis: usize) -> bool {
        let k = point.len() - 1;
        let axis = axis % k;

        if point[axis] > self.potency {
            // this entire subtree has no nodes dominating this entry
            return false;
        }

        // XXX: This should be an identity check.
        if self.location != point {
            // this node might beat the point
            if !self
                .location
                .iter()
                .take(k)
                .zip(point.iter())
                .any(|(loc, p)| loc <= p)
            {
                return true;
            }
        }

        // ok so this node doesn't beat the point. does it at least beat it
        // on the currently examined dimension?
        if self.location[axis] >= point[axis] {
            // yep, this means a total domination node can appear in both branches
            if let Some(ref node) = self.left {
                if node.dominates_axis(point, axis + 1) {
                    return true;
                }
            }
        }

        if let Some(ref node) = self.right {
            return node.dominates_axis(point, axis + 1);
        }

        false
    }
}

#[test]
fn test_kdtree() {
    let item_list = [
        // (foo, bar, baz)
        vec![10.0, 20.0, 10.0], // should get eliminated
        vec![10.0, 20.0, 30.0], // should stay because it's the only 30 baz that has 20 bar
        vec![30.0, 20.0, 10.0], // should stay because it's the only 30 foo that has 20 bar
        vec![30.0, 10.0, 30.0], // should stay because it's the only 30 foo that has 30 baz
        vec![10.0, 30.0, 20.0], // should stay because it's the only 30 bar that has 20 baz
        vec![30.0, 10.0, 20.0], // should get eliminated
        vec![20.0, 30.0, 10.0], // should stay because it's the only 30 bar that has 20 foo
    ];

    let eliminations = [0, 5];

    let mut item_list_clone = item_list.clone();
    let tree = KDTreeNode::from_points(&mut item_list_clone[..]).unwrap();

    for (i, item) in item_list.iter().enumerate() {
        assert!(eliminations.contains(&i) == tree.dominates(&item[..]));
    }
}

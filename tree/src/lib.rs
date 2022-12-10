pub mod lfr;

#[cfg(test)]
mod tests {
    use super::*;
    use lfr::tree::{Tree};

    #[test]
    fn tree_test() {
        let mut tree = Tree::new("/");
        
        let mut a_dir = Tree::new("a/");
        let mut b_dir = Tree::new("b/");
        let mut c_dir = Tree::new("c/");

        let u_file = Tree::new("u: 67");
        let v_file = Tree::new("v: 1556");
        let w_file = Tree::new("w: 345");
        let x_file = Tree::new("x: 6463");
        let y_file = Tree::new("y: 9621");
        let z_file = Tree::new("z: 523");

        c_dir.append(z_file);

        b_dir.append(x_file);
        b_dir.append(y_file);

        a_dir.append(b_dir);
        a_dir.append(v_file);
        a_dir.append(w_file);

        tree.append(u_file);
        tree.append(a_dir);
        tree.append(c_dir);

        print!("{}", tree);
    }
}

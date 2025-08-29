use crate::transpiler::InternalTree;

pub fn check_for_enter(tree: &InternalTree) -> Result<(), &'static str> {
    if let InternalTree::Enter(_) = tree {
        Ok(())
    } else {
        Err("Enter not found in internal tree. A <Root> should exist in the xml markup.")
    }
}

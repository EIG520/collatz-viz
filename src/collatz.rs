use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct TreeItem {
    pub xpos: u16,
    pub val: u128,
    pub dcount: u8,
}

impl Default for TreeItem {
    fn default() -> Self {
        TreeItem {xpos: 0, val: 1, dcount: 0}
    }
}

pub struct CollatzTree {
    // hella fragile and should NOT be public
    // but uhhh... I don't care fr
    pub numlist: Vec<TreeItem>,
    pub cols: HashMap<u16, TreeItem>,
    pub maxdcount: u8
}

impl Default for CollatzTree {
    fn default() -> Self {
        Self {numlist: vec![TreeItem::default()], cols: HashMap::from([(0, TreeItem::default())]), maxdcount: 3}
    }
}

impl CollatzTree {
    fn next_auto(&self) -> Option<TreeItem> {
        let last = self.numlist.last().unwrap();

        if (last.val - 1) % 3 == 0 && ((last.val - 1) / 3) % 2 == 1
            // ignore 4
            && last.val != 4
         {
            Some(TreeItem { xpos: last.xpos + 1, val: (last.val - 1) / 3, dcount: 0})
        
        // identify repetitive behavior
        } else if last.dcount >= self.maxdcount 
            // ignore powers of 2
            && !((last.val & (last.val-1)) == 0) {

            if last.xpos == 0 {return None}

            let npos = last.xpos - 1;

            let jlast = self.cols.get(&npos).unwrap();

            Some(TreeItem { xpos: npos, val: jlast.val.checked_mul(2)?, dcount: jlast.dcount + 1})
        } else {
            Some(TreeItem { xpos: last.xpos, val: last.val.checked_mul(2)?, dcount: last.dcount + 1})
        }
    }

    fn next_right(&self) -> Option<TreeItem> {
        let last = self.numlist.last().unwrap();

        if (last.val - 1) % 3 == 0 && ((last.val - 1) / 3) % 2 == 1 {
            Some(TreeItem { xpos: last.xpos + 1, val: (last.val - 1) / 3, dcount: 0})
        } else {
            Some(TreeItem { xpos: last.xpos, val: last.val.checked_mul(2)?, dcount: last.dcount + 1})
        }
    }

    fn next_left(&self) -> Option<TreeItem> {
        let last = self.numlist.last().unwrap();

        if last.xpos == 0 {return None}

        let npos = last.xpos - 1;
        let jlast = self.cols.get(&npos).unwrap();

        Some(TreeItem { xpos: npos, val: jlast.val.checked_mul(2)?, dcount: jlast.dcount + 1})
    }

    pub fn expand_with(&mut self, onext: Option<TreeItem>) -> bool {
        if let Some(next) = onext {
            self.cols.insert(next.xpos, next);
            self.numlist.push(next);
            true
        } else {
            false
        }
    }

    pub fn expand_auto(&mut self) -> bool {
        let next = self.next_auto();
        self.expand_with(next)
    }

    pub fn expand_right(&mut self) -> bool {
        let next = self.next_right();
        self.expand_with(next)
    }

    pub fn expand_left(&mut self) -> bool {
        let next = self.next_left();
        self.expand_with(next)
    }
}


use std::cmp;
use serde::Serialize;
use reqwest::Client;

#[derive(Serialize)]
struct SlackMessage {
    text: String,
}

#[derive(Debug, Clone, Default)]
struct Node {
    current_exposure: i64,
    max_limit: i64,
    lazy_add: i64,
}

pub struct LimitSegmentTree {
    tree: Vec<Node>,
    n: usize,
}

impl LimitSegmentTree {
    pub fn new(limits: Vec<i64>) -> Self {
        let n = limits.len();
        let tree = vec![Node::default(); 4 * n];
        let mut st = LimitSegmentTree { tree, n };
        if n > 0 {
            st.build(&limits, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, limits: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = Node {
                current_exposure: 0,
                max_limit: limits[start],
                lazy_add: 0,
            };
            return;
        }
        let mid = (start + end) / 2;
        self.build(limits, 2 * node, start, mid);
        self.build(limits, 2 * node + 1, mid + 1, end);
        self.push_up(node);
    }

    fn push_up(&mut self, node: usize) {
        self.tree[node].current_exposure = cmp::max(
            self.tree[2 * node].current_exposure,
            self.tree[2 * node + 1].current_exposure,
        );
        self.tree[node].max_limit = cmp::min(
            self.tree[2 * node].max_limit,
            self.tree[2 * node + 1].max_limit,
        );
    }

    fn push_down(&mut self, node: usize) {
        let lazy = self.tree[node].lazy_add;
        if lazy != 0 {
            self.apply(2 * node, lazy);
            self.apply(2 * node + 1, lazy);
            self.tree[node].lazy_add = 0;
        }
    }

    fn apply(&mut self, node: usize, val: i64) {
        self.tree[node].current_exposure += val;
        self.tree[node].lazy_add += val;
    }

    pub fn update_exposure(&mut self, l: usize, r: usize, val: i64) -> Result<(), String> {
        if self.n == 0 { return Ok(()); }
        self.update(1, 0, self.n - 1, l, r, val)
    }

    fn update(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, val: i64) -> Result<(), String> {
        if start > end || start > r || end < l {
            return Ok(());
        }

        if start >= l && end <= r {
            if self.tree[node].current_exposure + val > self.tree[node].max_limit {
                return Err(format!("VIOLAÇÃO DE LIMITE: Range [{}-{}], Carga Tentada: {}, Limite Disponível: {}",
                                   start, end, self.tree[node].current_exposure + val, self.tree[node].max_limit));
            }
            self.apply(node, val);
            return Ok(());
        }

        self.push_down(node);
        let mid = (start + end) / 2;
        self.update(2 * node, start, mid, l, r, val)?;
        self.update(2 * node + 1, mid + 1, end, l, r, val)?;
        self.push_up(node);
        Ok(())
    }
}

async fn notify_slack(webhook_url: &str, message: &str) {
    let client = Client::new();
    let msg = SlackMessage { text: message.to_string() };

    let _ = client.post(webhook_url)
        .json(&msg)
        .send()
        .await;
}

#[tokio::main]
async fn main() {
    let slack_url = "SUA_URL_DO_WEBHOOK_AQUI";
    let initial_limits = vec![1000, 2000, 1500, 3000, 5000];
    let mut monitor = LimitSegmentTree::new(initial_limits);

    let updates = vec![(0, 2, 800), (1, 4, 1500), (0, 0, 300)];

    for (l, r, val) in updates {
        if let Err(e) = monitor.update_exposure(l, r, val) {
            println!("Alerta: {}", e);
            notify_slack(slack_url, &format!("🚨 *Risk Alert* 🚨\n{}", e)).await;
        } else {
            println!("Update [{},{}] val:{} OK", l, r, val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_basic_limit() {
        let mut st = LimitSegmentTree::new(vec![100, 100]);
        assert!(st.update_exposure(0, 1, 50).is_ok());
        assert!(st.update_exposure(0, 0, 60).is_err()); // 50+60 > 100
    }

    proptest! {
        #[test]
        fn test_random_updates(limits in prop::collection::vec(1..10000i64, 1..100)) {
            let n = limits.len();
            let mut st = LimitSegmentTree::new(limits.clone());

            // Simula uma transação aleatória
            let l = 0;
            let r = n - 1;
            let val = 500;

            let res = st.update_exposure(l, r, val);

            // Verifica se a lógica da árvore condiz com o menor limite do array original
            let min_limit = limits.iter().min().unwrap();
            if val > *min_limit {
                assert!(res.is_err());
            }
        }
    }
}

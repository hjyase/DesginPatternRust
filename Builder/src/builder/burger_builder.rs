// 1. 修正拼写错误（BurgurCompenent → BurgerComponent，BottonBun → BottomBun）
#[derive(Debug)]
pub enum BurgerComponent {
    BottomBun,
    Patty,
    Tomato,
    Cheese,
    Lettuce,
    TopBun,
}

// 2. 为枚举实现 Display trait（解决打印报错）
impl std::fmt::Display for BurgerComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 直接用 Debug 格式打印（简单适配，无需自定义中文）
        write!(f, "{:?}", self)
    }
}

pub struct BurgerBuilder {
    // 同步修正枚举命名
    components: Vec<BurgerComponent>,
}

impl BurgerBuilder {
    pub fn new() -> Self {
        Self {
            // 同步修正枚举变体命名
            components: vec![BurgerComponent::BottomBun],
        }
    }

    // 3. 修正：接收 self（值），返回 Self（值），支持链式调用
    pub fn add_component(mut self, component: BurgerComponent) -> Self {
        self.components.push(component);
        self // 无需解引用，直接返回自身（值）
    }

    // 4. 修正：接收 self（值），返回 Self（值），解决引用错误
    pub fn build(self) -> Self {
        for c in self.components.iter() {
            println!("{}", c);
        }
        self // 直接返回自身（值）
    }
}

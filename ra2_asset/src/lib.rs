pub mod asset;
pub mod component;
pub mod loader;

use bevy::ecs::schedule::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum DebugGameState {
    #[default]
    /// 进入内容：
    ///     启动加载：地图、场景色盘、场景tile、场景shp、techno_rule（待补充）
    /// 期间：1. 用户等待其他玩家、配置角色信息（国家、颜色）、
    /// 配置对局信息（难度、基地车是否重复打包？）     2. 资源初始化。
    /// 不强制校验 离开条件：
    ///     用户点击“开始对局”。内容：初始对局的配置信息如：GameOpts
    ///     离开游戏、解散游戏。转至“Enter”
    Prepare,

    /// 进入条件：点击开始对局。
    /// 进入内容：
    ///     1. 用户资源初始化？
    /// 过程：
    ///     1. 初始资源：地图、场景色盘、场景tile、场景shp、techno_rule
    /// 离开条件：素材加载完毕，对局资源初始完毕。
    Loading,

    /// 游戏时间
    /// 进入内容：
    ///     地图、建筑、覆盖物
    ///     初始单位
    /// 过程操作内容：对局的交互
    /// 离开条件：对局结束
    PlayTime
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, States, Default)]
pub enum GameState {
    /// 点击打开游戏。
    /// 内容：加载资产、初始资源，如界面的图片等，待补充：
    ///     启动加载：待整理
    ///     加载完毕：country_rule/color_rule/各个地图简要信息（玩家数、
    /// 小地图）、界面的各个资源（待补充） 离开条件：部分资源初始完毕
    #[default]
    Open,

    /// 期间：待补充
    ///     1. ？
    /// 离开条件：加入游戏或者选定地图建立游戏，取决于玩家，不可控，
    /// 即资源加载不可控 离开内容：加载地图、场景信息、rule、art。
    /// 进入“Prepare”
    Enter,

    /// 进入内容：
    ///     启动加载：地图、场景色盘、场景tile、场景shp、techno_rule（待补充）
    /// 期间：1. 用户等待其他玩家、配置角色信息（国家、颜色）、
    /// 配置对局信息（难度、基地车是否重复打包？）     2. 资源初始化。
    /// 不强制校验 离开条件：
    ///     用户点击“开始对局”。内容：初始对局的配置信息如：GameOpts
    ///     离开游戏、解散游戏。转至“Enter”
    Prepare,

    /// 进入条件：点击开始对局。
    /// 进入内容：
    ///     1. 用户资源初始化？
    /// 过程：
    ///     1. 初始资源：地图、场景色盘、场景tile、场景shp、techno_rule
    /// 离开条件：素材加载完毕，对局资源初始完毕。
    Loading,

    /// 游戏时间
    /// 进入内容：
    ///     地图、建筑、覆盖物
    ///     初始单位
    /// 过程操作内容：对局的交互
    /// 离开条件：对局结束
    PlayTime,
    ///清理对局配置，进入Loading状态
    Clear
}

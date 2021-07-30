# 代码说明

## 阶段一目标

把走势分解为1分钟走势类型

## 流程

原始K线(Bar)  ->   包含处理过的K线 (Candle) -> 分型(Fx) -> 笔(Pen) -> 标准特征序列(Seq) -> 特征序列分型(SeqFx) -> 线段 (Sequence) ->
中枢(Pivot) -> 1分钟走势类型 (Trend)

其中Pen -> Seq -> SeqFx 可以类比与 Bar -> Candle -> Fx， 因此程序基本结构相同
Bar->Candle->Fx为了得到笔,同理Pen->Seq->SeqFx为了得到线段



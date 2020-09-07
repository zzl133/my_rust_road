# 自制搜索引擎

## ch1 搜索引擎是如何工作的

### 1-1 理解搜索引擎的构成

#### 什么是搜索引擎

搜索引擎是一类系统或软件的统称，作用是从文档的集合中查找（检索）出匹配信息需求（查询）的文档，信息需求是由单词、问题等构成的。

”全文搜索引擎”是指“全文”指的就是全部的句子，当检索的对象为“由文本构成的文档中的全部的句子”时，对于该文档进行得检索就称为全文搜索。而实现了这种全文搜索的系统就是全文搜索引擎
（全文搜索系统 Full-text Search Engine）


#### 构成搜索引擎的组件

搜索引擎一般由以下4个组件构成

- 索引管理系统 Index Manger 
- 索引检索器 Index Searcher
- 索引构建器 Indexer
- 文档管理器 Document Manger


* 索引管理器 

索引管理器组件的作用是管理带有索引结构的数据，索引结构是一种用于进行高速检索的数据结构。
对索引的访问也是通过索引管理器进行的。

索引管理器通常是将索引作为**二级存储**上的二进制文件来进行管理的。还经常会通过保存进行过压缩的索引来减少从二进制存储加载的数据量，提升检索处理效率的目的.

* 索引检索器

索引检索器是利用索引进行全文搜索处理的组件，索引检索器根据来自检索影城程序用户的查询，协同索引管理器进行检索处理。在大多数情况下，索引检索器都会根据某种标准对查询相匹配的检索结果排序，并将排在前面的结果返回给应用程序。

* 索引构建器

索引构建器是从作为检索对象的文本文档中生成索引的组件。所以构建器会先通过解析将文本
文档分解为单词序列，然后在将该单词序列转为索引结构。在搜索引擎中，将生成索引的环节称为索引构建(Index Construction)

* 文档管理器

文档管理器是管理文档数据库的组件，文档数据库中存储着作为检索对象的文档。 文档管理器会先从文档数据库中取出与查询相匹配的文档，然后再跟进需要从该文档中提取出一部分内容作为摘要。

文档管理器的结构很简单，只是对应着文档特定的ID来保存文档的内容。
有人将数据库管理系统和基于二级存储的数据库管理系统等用作完档管理系统。

由文档管理器管理的文档数据库既可以在构建索引的阶段随索引一同构建，也可以提前构建。



#### 与搜索引擎相关的组件

爬虫

爬虫是用于收集Web上Html文件等文档的系统。


搜索排序系统

以Google的PageRank系统为代表的搜索排序系统是给作为检索对象的问文档打分的系统。
例如在Web搜索中，通常会以考量了查询与文档的关联性以及文档的热门度后得出的分数为
基准，将检索结果排序后提供给应用程序的用户。搜索排序系统正是用于目的的，能（机械的）
算出文档热门度的系统。



### 1-2 实现快速全文搜索的索引结构

#### 全文搜索的两种方法

- 利用全扫描进行全文搜索

- 利用索引进行全文搜索

* 利用全扫描进行全文搜索

是从头到尾扫描作为检索对象的文档，以次来搜索要检索的字符串。 
缺点：只适用于处理少量或暂时性的文档
还有一些高级算法， KMP算法，BM算法。

* 利用索引进行全文搜索

利用索引的方法，则需要事先为文档建立索引，然后利用索引来搜索要检索的字符串
虽然需要事先建立索引需要花费时间，但是有点是即使文档的数量增加，检索速度也不会大幅下降。
因此，一般认为这种方法更适合处理大量的文档，搜索引起一般也会采用这种方法。

大部分的搜索引擎的索引结构采用的都是倒排索引的索引结构
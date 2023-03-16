# Rust
rust学习记录

1.冒泡排序

在bubble文件夹中中展示了冒泡排序使用的代码

fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {

    for i in 0..list.len() {
    
        for x in 0..list.len() - 1 {
        
            if list[x] > list[x + 1] {
            
                list.swap(x, x + 1); 
                
            }
            
        }
        
    }
    
    list
    
}

上面是冒泡函数代码撰写，之后书写一些测试数据进行测试

fn main() {

    let mut list = vec![1, 3, 4555, 24, 35, 24, 44, 2, 432];
    
    bubble_sort(&mut list);
    
    println!("{:?}  ", list);

    let mut list = vec!['g', 'h', 'T', 'R', 'b', 'f'];
    
    bubble_sort(&mut list);
    
    println!("{:?}  ", list);
    
}

可以运行，运行结果展示如下

![图片](https://user-images.githubusercontent.com/72017161/225773197-882c9fc7-35d2-40ea-a10a-2feecce5971b.png)

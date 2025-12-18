-- 1. 如果名为 'course' 的表存在，则删除它
drop table if exists course;

-- 2. 创建 'course' 表
create table course
(
    -- id: 主键，自增整数
    id serial primary key,
    -- teacher_id: 整数，非空
    teacher_id INT not null,
    -- name: 长度可变字符串，最大140字符，非空
    name varchar(140) not null,
    -- time: 时间戳，默认为当前时间
    time TIMESTAMP default now()
);

-- 3. 插入第一条数据
insert into course
    (id, teacher_id, name, time)
values
    (1, 1, 'First course', '2025-12-17 05:40:00');

-- 4. 插入第二条数据
insert into course
    (id, teacher_id, name, time)
values
    (2, 1, 'Second course', '2025-12-18 05:45:00');
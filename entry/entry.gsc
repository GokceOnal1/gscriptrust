blueprint Thing {
    prop list;
    method create(param l) {
        list = l;
    };
};
assign a = new Thing([1,2,new Thing(2)]);
write(a.list[2].list);
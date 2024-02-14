function y = Q1model(param,x)
    k1 = param(1);
    k2 = param(2);
    k3 = param(3);
    k4i = param(4);
    k4d = param(5);

    y_a = modelA(x,k1);
    y_b = modelB(x,k1,k2,k3);
    y_c = modelC(x,k1,k2,k3,k4i,k4d);
    y_d = modelD(x,k1,k2,k3,k4i,k4d);

    y = [y_a; y_b; y_c; y_d];
end
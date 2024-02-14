function y_c = modelC(x,k1,k2,k3,k4i,k4d)
    CA0 = 11.700;
    % k1 = 3.4654e-04;
    % k2 = 0.0132;
    % k3 = 0.0132;
    A = k2*k1*CA0;
    B = (k2+k3)*k4i*k1*CA0;
    r1 = 0;
    r2 = -(k4d+k4i);
    r3 = -k1;
    r4 = -(k2+k3);
    y_c = (A*r1+B)/((r1-r2)*(r1-r3)*(r1-r4))*exp(r1*x) + ...
        (A*r2+B)/((r2-r1)*(r2-r3)*(r2-r4))*exp(r2*x) + ...
        (A*r3+B)/((r3-r1)*(r3-r2)*(r3-r4))*exp(r3*x) + ...
        (A*r4+B)/((r4-r1)*(r4-r2)*(r4-r3))*exp(r4*x);
end
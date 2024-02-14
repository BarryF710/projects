function y_d = modelD(x,k1,k2,k3,k4i,k4d)
    CA0 = 11.700;
    C = k3*k1*CA0;
    D = k4d*k2*k1*CA0;
    E = k4d*(k2+k3)*k4i*k1*CA0;
    r1 = 0;
    r2 = -(k4d+k4i);
    r3 = -k1;
    r4 = -(k2+k3);
    r5 = -k4i;
    y_d = C*((r4-r5)*exp(r3*x)+(r5-r3)*exp(r4*x)+(r3-r4)*exp(r5*x))/((r3-r4)*(r4-r5)*(r5-r3)) + ...
        (D*r1+E)/((r1-r2)*(r1-r3)*(r1-r4)*(r1-r5))*exp(r1*x) + ...
        (D*r2+E)/((r2-r1)*(r2-r3)*(r2-r4)*(r2-r5))*exp(r2*x) + ...
        (D*r3+E)/((r3-r1)*(r3-r2)*(r3-r4)*(r3-r5))*exp(r3*x) + ...
        (D*r4+E)/((r4-r1)*(r4-r2)*(r4-r3)*(r4-r5))*exp(r4*x) + ...
        (D*r5+E)/((r5-r1)*(r5-r2)*(r5-r3)*(r5-r4))*exp(r5*x);
end
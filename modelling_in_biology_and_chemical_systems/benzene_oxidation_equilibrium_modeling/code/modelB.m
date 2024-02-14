function y_b = modelB(x,k1,k2,k3)
    CA0 = 11.700;
    % k1 = 3.4654e-04;
    y_b = k1*CA0/(-k1+(k2+k3))*(exp(-k1*x)-exp(-(k2+k3)*x));
end
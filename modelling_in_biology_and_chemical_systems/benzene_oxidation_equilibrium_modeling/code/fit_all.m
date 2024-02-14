clc
clear
close all

% first convert all concentration units to mM (1 mM = 1000 uM)

initialGuess = [0.13, 0.11, 0.12, 0.14, 0.15]; % initial guess for k1,k2,k3,k4i,k4d
lb = [0, 0, 0, 0, 0];  % Lower bounds for k1,k2,k3,k4i,k4d
ub = [0.5, 0.5, 0.5, 0.5, 0.5];  % Upper bounds for k1,k2,k3,k4i,k4d

x = [0; 30; 60; 90; 120; 180; 240]; % the time
y_a = [11.700; 11.574; 11.456; 11.341; 11.224; 10.993; 10.767]; % substance A concentration
y_b = [0; 78.994; 174.546; 276.537; 315.064; 416.034; 471.018]*1e-3; % substance B concentration
y_c = [0; 22.891; 46.705; 53.960; 68.577; 84.932; 96.685]*1e-3; % substance C concentration
y_d = [0; 11.853; 35.669; 96.270; 145.833; 221.053; 376.401]*1e-3; % substance D concentration
y = [y_a; y_b; y_c; y_d];
options = optimoptions('lsqcurvefit', 'Algorithm', 'trust-region-reflective');
opt_param = lsqcurvefit(@Q1model, initialGuess, x, y, lb, ub, options);

opt_k1 = opt_param(1);
opt_k2 = opt_param(2);
opt_k3 = opt_param(3);
opt_k4i = opt_param(4);
opt_k4d = opt_param(5);

y_a_pred = zeros(size(x));
y_b_pred = zeros(size(x));
y_c_pred = zeros(size(x));
y_d_pred = zeros(size(x));

for i = 1:length(x)
    y_a_pred(i) = modelA(x(i),opt_k1);
    y_b_pred(i) = modelB(x(i),opt_k1,opt_k2,opt_k3);
    y_c_pred(i) = modelC(x(i),opt_k1,opt_k2,opt_k3,opt_k4i,opt_k4d);
    y_d_pred(i) = modelD(x(i),opt_k1,opt_k2,opt_k3,opt_k4i,opt_k4d);
end

figure;
plot(x,y_a_pred,'LineWidth',2,'DisplayName','A predicted');
hold on
scatter(x,y_a,'filled','DisplayName','A actual');
hold off
legend('show')
xlabel('Time')
ylabel('Concentration')

figure;
plot(x,y_b_pred,'LineWidth',2,'DisplayName','B predicted');
hold on
scatter(x,y_b,'filled','DisplayName','B actual');
hold off
legend('show')
xlabel('Time')
ylabel('Concentration')

figure;
plot(x,y_c_pred,'LineWidth',2,'DisplayName','C predicted');
hold on
scatter(x,y_c,'filled','DisplayName','C actual');
hold off
legend('show')
xlabel('Time')
ylabel('Concentration')

figure;
plot(x,y_d_pred,'LineWidth',2,'DisplayName','D predicted');
hold on
scatter(x,y_d,'filled','DisplayName','D actual');
hold off
legend('show')
xlabel('Time')
ylabel('Concentration')

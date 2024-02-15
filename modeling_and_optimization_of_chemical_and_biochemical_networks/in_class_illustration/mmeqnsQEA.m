function yp=mmeqnsQEA(t,y,par)
k1=par(1);kneg1=par(2);k2=par(3);S0=par(4);E0=par(5);
S=y(1);ES=y(2);E=y(3);P=y(4);
Kd=(kneg1)/k1;

b=E0+Kd+y(4)-S0; 
c=Kd*(y(4)-S0);
S=(-b+sqrt(b.^2-4.*c))./2;

ES=E0*S/(Kd+S);
E=E0-ES;
yp(1,1)=0;
yp(2,1)=0;
yp(3,1)=0;
yp(4,1)=k2*(ES+E)*S/(Kd+S);

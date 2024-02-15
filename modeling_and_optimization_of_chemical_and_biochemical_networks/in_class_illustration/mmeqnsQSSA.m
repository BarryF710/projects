function yp=mmeqnsQSSA(t,y,par)
k1=par(1);kneg1=par(2);k2=par(3);
S=y(1);ES=y(2);E=y(3);P=y(4);
Km=(kneg1+k2)/k1;
yp(1,1)=-k2*(ES+E)*S/(Km+S);
yp(2,1)=0;
yp(3,1)=0;
yp(4,1)=k2*(ES+E)*S/(Km+S);

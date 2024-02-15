function yp=mmeqns(t,y,par)
k1=par(1);kneg1=par(2);k2=par(3);
S=y(1);ES=y(2);E=y(3);P=y(4);

yp(1,1)=-k1*E*S + kneg1*ES;
yp(2,1)=k1*E*S- (kneg1+k2)*ES;
yp(3,1)=-k1*E*S+ (kneg1+k2)*ES;
yp(4,1)=k2*ES;

%Simulation of Michaelis-Menten Kinetics from Tsuchiya
% or the Klipp text book
%y =[s x e p]

% Case 1 % S>>>E eo=1,Kd=1,Km=1.01 Km=(k-1+k2)/k1
%    y0=[10,0,1,0]; %S >>> E 10 fold
%   
%   k1=100;kneg1=100;k2=1; %QEA/QSSA Assumption holds

% Case 2 S>>>E eo=1,Km= 2
%   y0=[10,0,1,0]; %S >>> E 10 fold
%  k1=1;kneg1=1;k2=1; %QEA/QSSA OK except for initial stage
%  
% Case 3 S~==E, Km= 2
%   y0=[1,0,1,0]; %S is not >>> E
%   k1=1;kneg1=1;k2=1; %Both are bad for initial transience (eo/Km=1/2~=1)
% %  
% Case 4 S~=E, Km= 101
   y0=[1,0,1,0]; %S is not >>> E
    k1=1;kneg1=1;k2=100; %QEA Breaks down QSSA is OK (eo/Km=1/101<<1)


par(1)=k1;par(2)=kneg1;par(3)=k2;par(4)=y0(:,1);par(5)=y0(:,3);
[t,y]=ode23s(@mmeqns,[0 10],y0,[],par);
[tqs,yqs]=ode23s(@mmeqnsQSSA,[0 10],y0,[],par);
[tqe,yqe]=ode23s(@mmeqnsQEA,[0 10],y0,[],par);

%QSSA
Km=(kneg1+k2)/k1;
yqs(:,2)=y0(3).*yqs(:,1)./(Km+yqs(:,1));
yqs(:,3)=y0(3)-yqs(:,2);

%QEA
Kd=(kneg1)/k1;
b=y0(3)+Kd+yqe(:,4)-y0(1); 
c=Kd.*(yqe(:,4)-y0(1));
yqe(:,1)=(-b+sqrt(b.^2-4.*c))./2;
yqe(:,2)=y0(3).*yqe(:,1)./(Kd+yqe(:,1));
yqe(:,3)=y0(3)-yqe(:,2);


subplot(3,1,1),plot(t,y(:,1),'r',t,y(:,2),'g',t,y(:,3),'c',t,y(:,4),'k');
legend('S','ES','E','P');ylabel('Conc.');xlabel('Time (s)');title('Exact');
subplot(3,1,2),plot(tqs,yqs(:,1),'r',tqs,yqs(:,2),'g',tqs,yqs(:,3),'c',tqs,yqs(:,4),'k');
legend('S','ES','E','P');ylabel('Conc.');xlabel('Time (s)');title('QSSA');
subplot(3,1,3),plot(tqe,yqe(:,1),'r',tqe,yqe(:,2),'g',tqe,yqe(:,3),'c',tqe,yqe(:,4),'k');
legend('S','ES','E','P');ylabel('Conc.');xlabel('Time (s)');title('QEA');
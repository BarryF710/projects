%File to illustrate the fundamental subspace of 
%stochiometric spaces of S
%Looking at reactions


%A+B->C+D
%B+D->E
%E+B->2C
%A+D->2C
%A+C->E+D
%A+E->2C+B

%Metabs: A(6 carbon), B (3c),C (5c), D (4 c), E (7 c)
S=[-1,0,0,-1,-1,-1;-1,-1,-1,0,0,1;1,0,2,2,-1,2;1,-1,0,-1,1,0;0,1,-1,0,1,-1];
S
size(S)
pause;
display('Rank');
rank(S)
pause

%Right Null space
RNS=null(S,'r');
display('Dimension of Right Null Space');
length(RNS(1,:))
pause;

%Left Null Space
Laa=null(S','r');
display('Dimension of Left Null Space');
rats(Laa)

display('Surprise, Surprise');
pause

%You can SVD to identify the subspaces of S
[u, singv, v]=svd(S);

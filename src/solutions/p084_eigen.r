A <- read.csv("trans.csv", header = FALSE)
v = Re(eigen(t(A))$vectors[,1])
cat(v / sum(v))

require(ggplot2)
require(reshape2)

data <- read.table("mesures.dat", header = T)
m <- melt(data, id.vars = c("n", "version"))
summary(m)

ggplot(data = m, aes(x = n, y = value)) +
    facet_grid(variable ~ version, scales = "free", switch = "y") +
    geom_point() +
    geom_line() +
    ylab("") +
    theme_bw()


ggsave("mesures.svg", width = 5, height = 5)


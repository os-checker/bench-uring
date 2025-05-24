library(ggplot2)
library(gridExtra)
library(stringr)
library(dplyr)
#library(DT)
# 自动应用中文字体
showtext::showtext_auto()

# 设置默认主题
font_ratio = 1
theme_set(
  theme_minimal() +
    theme(
      text = element_text(size = 16*font_ratio),  # 设置全局字体大小为 16
      plot.title = element_text(size = 20*font_ratio, face = "bold"),
      axis.title = element_text(size = 16*font_ratio),
      axis.text = element_text(size = 14*font_ratio),
      legend.title = element_text(size = 16*font_ratio),
      legend.text = element_text(size = 14*font_ratio)
    )
)

human_readable_bytes <- function(bytes) {
  units <- c("B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB")
  if (bytes == 0) return("0 B")
  k <- 1024
  magnitude <- floor(log(bytes, k))
  value <- bytes / (k^magnitude)
  return(sprintf("%d %s", value, units[magnitude + 1]))
}
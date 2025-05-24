# 去除 server 列中的 "server_" 前缀
filtered_data$server <- sub("^server_", "", filtered_data$server)
filtered_data$client <- sub("^client_", "", filtered_data$client)
# 修改标签格式
filtered_data$server <- sub("_st$", "\n(1 thread)", filtered_data$server)
filtered_data$server <- sub("_mt$", "\n(8 threads)", filtered_data$server)
filtered_data$client <- sub("_st$", " (1 thread)", filtered_data$client)
filtered_data$client <- sub("_mt$", " (8 threads)", filtered_data$client)

filtered_data <- filtered_data %>%
  group_by(server, client) %>%
  summarise(mbps = mean(mbps, na.rm = TRUE), .groups = 'drop')

# 按 mbps 降序排列
filtered_data <- filtered_data %>%
  arrange(desc(mbps))

p <- ggplot(filtered_data, aes(
    x = reorder(server, -mbps),
    y = mbps
  )) +
    geom_bar(stat = "identity", position = "dodge", fill = color) +
    labs(
      title = title,
      x = "Server",
      y = "MBps",
    ) +
    geom_text(
      aes(
        label = round(mbps, 0),
        fontface = "bold"
      ),
      color = color,
      position = position_dodge(width = 0.9),
      vjust = -0.4,
      size = 2.8,
      show.legend = FALSE
    ) +
    theme_minimal() +
    theme(
      axis.text.x = element_text(angle = 0),
      # 如果标签过长，可以旋转标签
      legend.direction = "horizontal",
      legend.position = "inside",
      # 将图例放入绘图区
      legend.position.inside = c(0.7, 0.88),
      legend.background = element_rect(fill = "white", color = NA),
    ) +
    scale_y_continuous(expand = c(0.08, 0)) # 扩展 y 轴范围，为标签提供更多空间
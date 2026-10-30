#!/usr/bin/env bash
# S1 自动扫描 —— 全程只读，不调用任何游戏方法（不碰 invoke_static）。
# 用法：
#   手机 Termux:  bash s15_probe.sh
#   电脑:         adb forward tcp:18765 tcp:18765 && bash s15_probe.sh
# 可覆盖：BASE=http://127.0.0.1:18765 OUT=s15 bash s15_probe.sh
set -u

BASE="${BASE:-http://127.0.0.1:18765}"
OUT="${OUT:-s15}"
KEYWORDS="Festa ScenarioFesta SummerOuting Outing FailureRate FailRate BuffEffect PotentialIncDec Slot Equip Gauge Advice"

mkdir -p "$OUT" || exit 1

# --- 0. 通不通 ---
if ! curl -s --max-time 10 "$BASE/health" -o "$OUT/00_health.json"; then
  echo "插件不通：$BASE —— 游戏没开 / 没注入 / 没做 adb forward"
  exit 1
fi
# 注意：插件的 JSON 是 format! 拼出来的紧凑格式，但这里一律容忍空格，
# 免得哪天改成 pretty-print 就整个脚本判「插件异常」。
grep -qE '"status"[[:space:]]*:[[:space:]]*"ok"' "$OUT/00_health.json" || {
  echo "health 异常：$(head -c 200 "$OUT/00_health.json")"; exit 1; }
echo "插件 OK：$(grep -oE '"version"[[:space:]]*:[[:space:]]*"[^"]*"' "$OUT/00_health.json" | head -1)"
echo

# --- 1. 类名存活扫描 ---
printf '%-18s %6s %10s\n' KEYWORD HITS SINGLETON
for k in $KEYWORDS; do
  f="$OUT/10_classes_$k.json"
  curl -s --max-time 90 "$BASE/il2cpp/classes?keyword=$k" -o "$f"
  if grep -q '"error"' "$f"; then
    printf '%-18s %6s %10s   %s\n' "$k" "-" "-" "$(head -c 60 "$f")"
    continue
  fi
  hits=$(grep -o '"name"' "$f" | wc -l | tr -d ' ')
  sing=$(grep -oE '"singleton"[[:space:]]*:[[:space:]]*true' "$f" | wc -l | tr -d ' ')
  printf '%-18s %6s %10s\n' "$k" "$hits" "$sing"
  # 命中少的时候直接把类名列出来，方便挑锚点
  if [ "$hits" != "0" ] && [ "$hits" -le 12 ]; then
    grep -oE '"name"[[:space:]]*:[[:space:]]*"[^"]*"' "$f" | sed 's/.*:[[:space:]]*"/    /; s/"$//'
  fi
done
echo

# --- 2. 锚点类的方法清单（只取 WorkSingleMode / Service / Effect 族，最多 8 个）---
echo "拉取方法清单（/il2cpp/methods）："
for k in Festa FailureRate BuffEffect SummerOuting; do
  f="$OUT/10_classes_$k.json"
  [ -s "$f" ] || continue
  grep -oE '"name"[[:space:]]*:[[:space:]]*"[^"]*"' "$f" | sed 's/.*:[[:space:]]*"//; s/"$//' \
    | grep -E 'WorkSingleMode|Service|Effect|FailureRate' | sort -u | head -3 \
    | while read -r cls; do
        o="$OUT/20_methods_$cls.json"
        curl -s --max-time 60 "$BASE/il2cpp/methods?name=$cls" -o "$o"
        n=$(grep -oE '"name"[[:space:]]*:' "$o" | wc -l | tr -d ' ')
        printf '  %-52s %s methods\n' "$cls" "$n"
      done
done
echo
echo "产物在 $OUT/ —— 打包后用 Workbench 文件投递传上来。"
echo "下一步看采集单 S3：singleton:true 的类才可以用 /il2cpp/call 当黑盒问。"

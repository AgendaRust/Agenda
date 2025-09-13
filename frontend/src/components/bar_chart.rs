use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;
use yew::{function_component, html, use_effect_with, use_node_ref, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct BarChartProps {
    pub data: Vec<(String, f64)>,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[function_component(BarChart)]
pub fn bar_chart(props: &BarChartProps) -> Html {
    let canvas_ref = use_node_ref();

    // Clones para uso dentro do efeito
    let canvas_ref_for_effect = canvas_ref.clone();
    let data = props.data.clone();
    let title = props.title.clone();
    let width = props.width;
    let height = props.height;

    // Uso correto de use_effect_with
    use_effect_with(
        // Primeiro parâmetro: dependências
        (data.clone(), title.clone()),
        // Segundo parâmetro: closure que recebe referência às dependências
        move |_deps| {
            if let Some(canvas) = canvas_ref_for_effect.cast::<HtmlCanvasElement>() {
                canvas.set_width(width);
                canvas.set_height(height);

                if let Ok(context) = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                {
                    draw_bar_chart(&context, &data, &title, width, height);
                }
            }

            // Retorna função de cleanup
            || ()
        }
    );

    html! {
        <canvas
            ref={canvas_ref}
            width={props.width.to_string()}
            height={props.height.to_string()}
            class="bar-chart-canvas"
        />
    }
}

// Implementação da função draw_bar_chart que estava faltando
fn draw_bar_chart(
    context: &CanvasRenderingContext2d,
    data: &[(String, f64)],
    title: &str,
    canvas_width: u32,
    canvas_height: u32,
) {
    let width = canvas_width as f64;
    let height = canvas_height as f64;

    // Limpar canvas
    context.clear_rect(0.0, 0.0, width, height);

    // Configurações do gráfico
    let margin = 60.0;
    let chart_width = width - 2.0 * margin;
    let chart_height = height - 2.0 * margin - 40.0; // Espaço extra para título

    if data.is_empty() {
        return;
    }

    // Encontrar valor máximo
    let max_value = data.iter().map(|(_, v)| *v).fold(0.0, f64::max);

    // Desenhar título
    context.set_font("16px Arial");
    context.set_fill_style(&"#333".into());
    context.set_text_align("center");
    context.fill_text(title, width / 2.0, 30.0).unwrap();

    // Configurações das barras
    let bar_width = chart_width / data.len() as f64 * 0.8;
    let bar_spacing = chart_width / data.len() as f64 * 0.2;

    // Desenhar barras
    for (i, (label, value)) in data.iter().enumerate() {
        let x = margin + i as f64 * (bar_width + bar_spacing);
        let bar_height = (value / max_value) * chart_height;
        let y = margin + 40.0 + chart_height - bar_height;

        // Cor da barra (alternando cores)
        let color = if i % 2 == 0 { "#4CAF50" } else { "#2196F3" };
        context.set_fill_style(&color.into());
        context.fill_rect(x, y, bar_width, bar_height);

        // Valor acima da barra
        context.set_font("12px Arial");
        context.set_fill_style(&"#333".into());
        context.set_text_align("center");
        context.fill_text(
            &value.to_string(),
            x + bar_width / 2.0,
            y - 5.0
        ).unwrap();

        // Label embaixo da barra
        context.set_font("10px Arial");
        context.fill_text(
            label,
            x + bar_width / 2.0,
            margin + 40.0 + chart_height + 20.0
        ).unwrap();
    }

    // Desenhar eixos
    context.set_stroke_style(&"#333".into());
    context.set_line_width(1.0);
    context.begin_path();

    // Eixo Y
    context.move_to(margin, margin + 40.0);
    context.line_to(margin, margin + 40.0 + chart_height);

    // Eixo X
    context.move_to(margin, margin + 40.0 + chart_height);
    context.line_to(margin + chart_width, margin + 40.0 + chart_height);

    context.stroke();
}
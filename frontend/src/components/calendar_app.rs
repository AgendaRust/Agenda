use yew::{function_component, html, Html};


#[function_component(CalendarApp)]
pub fn calendar_app() -> Html {
    
    
    
    html! {
        <div class="calendar-app">
            <div class="calendar">
                <h2 class="calendar-heading">{ "Calendar" }</h2>
                <div class="navigate-date">
                    <h2 class="month"> { "October" } </h2>
                    <h2 class="year"> { "2023" } </h2>
                    <div class="calendar-buttons">
                        <button>{ "<" }</button>
                        <button>{ ">" }</button>
                    </div>
                    // Calendar grid implementation goes here
                </div>
                <div class="weekdays">
                    <span class="weekday">{ "Sun" }</span>
                    <span class="weekday">{ "Mon" }</span>
                    <span class="weekday">{ "Tue" }</span>
                    <span class="weekday">{ "Wed" }</span>
                    <span class="weekday">{ "Thu" }</span>
                    <span class="weekday">{ "Fri" }</span>
                    <span class="weekday">{ "Sat" }</span>
                </div>
                <div class="days">
                    // Days of the month will be rendered here
                    { for (1..=31).map(|day| html! {
                        
                            <span class="unique-day">{ day }</span>
                            // Tasks for the day can be listed here
                        
                    }) }
                    <span class="current-day"> { "32" } </span>
                </div>
                <div class="events">
                    <div class="task-popup">
                        <div class="time-input">
                            <div class="event-popup-time">
                                <input type="number" name="hours" min="0" max = "23" class="hour-input" placeholder="HH" />
                                { ":" }
                                <input type="number" name="minutes" min="0" max = "59" class="minute-input" placeholder="MM" />
                            </div>
                            // <textarea placeholder="Enter event details..." class="event-popup-details"></textarea>
                        </div>
                        
                        <label for="title">{ "Nova task:" }</label>
                        <input 
                            type="text" 
                            id="title" 
                            name="title" 
                            minlength="3" 
                            required=true 
                            placeholder="Digite o título da task"
                        />
                        
                        <label for="category">{ "Categoria:" }</label>
                        <input 
                            type="text" 
                            id="category" 
                            name="category" 
                            minlength="5" 
                            required=true 
                            placeholder="Digite a categoria"
                        />
                        
                        <label for="description">{ "Descrição:" }</label>
                        <textarea 
                            id="description" 
                            name="description" 
                            required=true 
                            placeholder="Digite a descrição"
                            rows="3"
                        ></textarea>
                        
                        <label for="begin_date">{ "Data de Início:" }</label>
                        <input 
                            type="datetime-local" 
                            id="begin_date" 
                            name="begin_date" 
                            required=true 
                        />

                        <label for="type">{ "Tipo:" }</label>
                        <select 
                            id="type" 
                            name="type" 
                            required=true 
                        >
                            <option value="MeiaHora">{ "Meia Hora" }</option>
                            <option value="UmaHora">{ "Uma Hora" }</option>
                            <option value="Manhã">{ "Manhã" }</option>
                            <option value="Tarde">{ "Tarde" }</option>
                            <option value="Noite">{ "Noite" }</option>
                        </select>
                        
                        <div class="popup-buttons">
                            <button class="event-popup-save">{"Add Task"}</button>
                            <button class="event-popup-cancel">{"Cancel"}</button>
                        </div>
                    </div>
                    <div class="task">
                        <div class="task-date-wrapper">
                            <div class="task-date"> { "May 20, 2023" } </div>
                            <div class="task-time"> { "10:00 - 11:00" } </div>
                        </div>
                        <div class="task-title"> { "Meeting with Team" } </div>
                        <div class="task-buttons">
                            <button class="task-edit"> { "Edit" } </button>
                            <button class="task-delete"> { "Delete" } </button>
                        </div>
                    </div>
                </div>
                
            </div>
        </div>
    }
}
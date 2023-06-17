#include "Fl_Ext.hpp"
#include "Fl_Flow.hpp"

// window extensions
template <typename Win_T> //
class Fl_Win_Ext : public Fl_Ext<Win_T>
{
  private:
    Fl_Flow flow_;

  public:
    inline Fl_Win_Ext(int X, int Y, int W, int H, const char* L = 0);

    inline void rule(Fl_Widget* widget, std::string instruction);
    inline void rule(Fl_Widget& widget, std::string instruction);
};

// implementations

template <typename Win_T>
inline Fl_Win_Ext<Win_T>::Fl_Win_Ext(int X, int Y, int W, int H, const char* L) //
    : Fl_Ext<Win_T>(X, Y, W, H, L),                                             //
      flow_(0, 0, sdpi(W), sdpi(H))
{
    flow_.end();
    this->end();

    this->add(flow_);
    this->resizable(flow_);
}

template <typename Win_T>
inline void Fl_Win_Ext<Win_T>::rule(Fl_Widget* widget, std::string instruction)
{
    flow_.rule(widget, instruction);
}

template <typename Win_T>
inline void Fl_Win_Ext<Win_T>::rule(Fl_Widget& widget, std::string instruction)
{
    flow_.rule(widget, instruction);
}
